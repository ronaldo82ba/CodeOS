use codecore::ipc::{IpcMessage, IpcMessageKind, shared_bus};
use codecore::init_core;
use codeos_ipc_client::{stubs::app, IpcClient, IpcError};
use codesvc_appmgr::register_ipc_endpoint_on;
use serde_json::json;

#[test]
fn ipc_request_response_roundtrip() {
    init_core();
    let bus = shared_bus();
    bus.lock()
        .expect("ipc bus lock poisoned")
        .register_endpoint(
            "test.echo".into(),
            Box::new(|msg| Some(IpcMessage::new_response(&msg, json!({ "echo": true })))),
        );

    let payload = bus
        .lock()
        .expect("ipc bus lock poisoned")
        .send_request("test.client", "test.echo", "ping", json!({}))
        .expect("roundtrip");
    assert_eq!(payload["echo"], true);
}

#[test]
fn ipc_event_delivery() {
    init_core();
    let bus = shared_bus();
    let seen = std::sync::Arc::new(std::sync::Mutex::new(String::new()));
    let seen_in_handler = std::sync::Arc::clone(&seen);

    bus.lock()
        .expect("ipc bus lock poisoned")
        .register_endpoint(
            "test.event_sink".into(),
            Box::new(move |msg| {
                if msg.kind == IpcMessageKind::Event {
                    *seen_in_handler.lock().unwrap() = msg.method.clone();
                }
                None
            }),
        );

    let client = IpcClient::new("test.client");
    client.event("test.event_sink", "Test.Event", json!({}));
    assert_eq!(*seen.lock().unwrap(), "Test.Event");
}

#[test]
fn ipc_unknown_endpoint_error_handling() {
    init_core();
    let client = IpcClient::new("test.client");
    let err = client
        .request("missing.endpoint", "ping", json!({}))
        .unwrap_err();
    assert!(matches!(err, IpcError::ServiceError(_)));
}

#[test]
fn ipc_correlation_id_matching() {
    init_core();
    let bus = shared_bus();
    bus.lock()
        .expect("ipc bus lock poisoned")
        .register_endpoint(
            "test.correlate".into(),
            Box::new(|msg| Some(IpcMessage::new_response(&msg, json!({ "request_id": msg.id })))),
        );

    let client = IpcClient::new("test.client");
    let payload = client
        .request("test.correlate", "ping", json!({}))
        .expect("correlated response");
    assert!(payload.get("request_id").and_then(|v| v.as_str()).is_some());
}

#[test]
fn app_manager_start_app_via_client() {
    use std::sync::{Arc, Mutex};

    use codecore::ipc::IpcBus;
    use codesvc_pkg::{register_ipc_endpoint_on as register_pkg, AppSection, CappPackager, CodeOsManifest, EntrySection};

    let apps = std::env::temp_dir().join(format!("codeos-roundtrip-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&apps);
    std::fs::create_dir_all(&apps).expect("create apps dir");
    std::env::set_var("CODEOS_APPS_DIR", &apps);

    init_core();
    let bus = Arc::new(Mutex::new(IpcBus::new()));
    register_ipc_endpoint_on(&bus);
    register_pkg(&bus);

    let fixture_dir = apps.join("fixtures");
    std::fs::create_dir_all(&fixture_dir).expect("fixture dir");
    let manifest = CodeOsManifest {
        app: AppSection {
            id: "com.ronaldo.otherapp".into(),
            name: "Other App".into(),
            version: "1.0.0".into(),
            min_os_version: "0.1.0".into(),
        },
        entry: EntrySection {
            binary: "bin/app".into(),
            args: vec![],
        },
        ui: None,
        permissions: Default::default(),
        metadata: None,
    };
    let capp_path = fixture_dir.join("other.capp");
    CappPackager::pack(&manifest, &capp_path, &[("bin/app", b"stub")]).expect("pack");

    let client = IpcClient::with_bus("app:com.ronaldo.myapp", bus.clone());
    client
        .request(
            "codesvc.pkg",
            "Pkg.Install",
            json!({ "capp_path": capp_path.to_string_lossy() }),
        )
        .expect("install app");

    let payload = client
        .request(
            app::SERVICE_ID,
            app::methods::START_APP,
            json!({ "app_id": "com.ronaldo.otherapp" }),
        )
        .expect("start app");
    assert!(payload.get("instance_id").and_then(|v| v.as_str()).is_some());

    let _ = std::fs::remove_dir_all(&apps);
}
