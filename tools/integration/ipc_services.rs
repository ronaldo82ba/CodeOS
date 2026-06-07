use codecore::init_core;
use codecore::ipc::{error_codes, IpcBus};
use codeos_ipc_client::{
    stubs, AppManagerClient, AppManagerService, AuthService, AuthServiceClient,
    CheckPermissionRequest, CreateSurfaceRequest, EventSubscription, GetAppInfoRequest,
    InstallRequest, IpcClient, IpcError, ListFilesRequest, NotificationService,
    NotificationServiceClient, PackageService, PackageServiceClient, PostNotificationRequest,
    ReadFileRequest, RequestPermissionRequest, StartAppRequest, StorageService,
    StorageServiceClient, SubmitFrameRequest, WindowService, WindowServiceClient,
    WriteFileRequest,
};
use codesvc_window::{set_frame_sink, CallbackFrameSink};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use codesvc_pkg::{get_registry, AppSection, CappPackager, CodeOsManifest, EntrySection};

static TEST_APPS_COUNTER: AtomicU64 = AtomicU64::new(0);

fn test_apps_dir(name: &str) -> PathBuf {
    let n = TEST_APPS_COUNTER.fetch_add(1, Ordering::SeqCst);
    std::env::temp_dir().join(format!(
        "codeos-services-{name}-{}-{n}",
        std::process::id()
    ))
}

fn install_test_app(client: &IpcClient, app_id: &str, apps_dir: &PathBuf) {
    let fixture_dir = apps_dir.join("fixtures");
    std::fs::create_dir_all(&fixture_dir).expect("fixture dir");
    let manifest = CodeOsManifest {
        app: AppSection {
            id: app_id.into(),
            name: "Test App".into(),
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
    let capp_path = fixture_dir.join(format!("{app_id}.capp"));
    CappPackager::pack(&manifest, &capp_path, &[("bin/app", b"stub")]).expect("pack test app");
    PackageServiceClient::new(client.clone())
        .install(InstallRequest {
            capp_path: capp_path.to_string_lossy().into_owned(),
        })
        .expect("install test app");
}

fn setup_services() -> IpcClient {
    let apps = test_apps_dir("setup");
    let _ = std::fs::remove_dir_all(&apps);
    std::fs::create_dir_all(&apps).expect("create apps dir");
    std::env::set_var("CODEOS_APPS_DIR", &apps);

    init_core();
    codecore::ipc::reset_event_subscribers();
    get_registry()
        .lock()
        .expect("registry lock")
        .clear();
    let bus = Arc::new(Mutex::new(IpcBus::new()));
    codesvc_window::register_ipc_endpoint_on(&bus);
    codesvc_appmgr::register_ipc_endpoint_on(&bus);
    codesvc_pkg::register_ipc_endpoint_on(&bus);
    codesvc_notif::register_ipc_endpoint_on(&bus);
    codesvc_auth::register_ipc_endpoint_on(&bus);
    codesvc_storage::register_ipc_endpoint_on(&bus);
    IpcClient::with_bus("test.client", bus)
}

fn setup_services_with_installed_app() -> (IpcClient, String) {
    let client = setup_services();
    let app_id = format!(
        "com.test.app.{}",
        TEST_APPS_COUNTER.fetch_add(1, Ordering::SeqCst)
    );
    let apps = PathBuf::from(std::env::var("CODEOS_APPS_DIR").expect("apps dir"));
    install_test_app(&client, &app_id, &apps);
    (client, app_id)
}

#[test]
fn window_create_surface() {
    let client = setup_services();
    let window = WindowServiceClient::new(client);
    let resp = window
        .create_surface(CreateSurfaceRequest {
            app_id: "com.test.app".into(),
            width: 640,
            height: 480,
        })
        .expect("create surface");
    assert!(!resp.surface_id.is_empty());
}

#[test]
fn window_submit_frame_delivers_to_sink() {
    let delivered = Arc::new(AtomicBool::new(false));
    let flag = Arc::clone(&delivered);
    set_frame_sink(Arc::new(CallbackFrameSink::new(
        move |surface_id, app_id, width, height, frame_data| {
            assert!(!surface_id.is_empty());
            assert_eq!(app_id, "com.test.app");
            assert_eq!(width, 640);
            assert_eq!(height, 480);
            assert_eq!(frame_data, "aGVsbG8=");
            flag.store(true, Ordering::SeqCst);
        },
    )));

    let client = setup_services();
    let window = WindowServiceClient::new(client);
    let resp = window
        .create_surface(CreateSurfaceRequest {
            app_id: "com.test.app".into(),
            width: 640,
            height: 480,
        })
        .expect("create surface");
    window.submit_frame(SubmitFrameRequest {
        surface_id: resp.surface_id,
        frame_data: "aGVsbG8=".into(),
    });
    assert!(delivered.load(Ordering::SeqCst));
}

#[test]
fn window_surface_changed_event_on_create() {
    let client = setup_services();
    let sub = EventSubscription::subscribe(stubs::window::events::SURFACE_CHANGED);
    let window = WindowServiceClient::new(client);
    let resp = window
        .create_surface(CreateSurfaceRequest {
            app_id: "com.test.app".into(),
            width: 800,
            height: 600,
        })
        .expect("create surface");

    let event = sub
        .recv_timeout(std::time::Duration::from_secs(1))
        .expect("surface changed event");
    assert_eq!(event.method, stubs::window::events::SURFACE_CHANGED);
    assert_eq!(event.payload["surface_id"], resp.surface_id);
    assert_eq!(event.payload["width"], 800);
    assert_eq!(event.payload["height"], 600);
}

#[test]
fn app_manager_start_stop_and_list() {
    let (client, app_id) = setup_services_with_installed_app();
    let apps = AppManagerClient::new(client);
    let started = apps
        .start_app(StartAppRequest { app_id })
        .expect("start");
    let list = apps.list_running().expect("list");
    assert_eq!(list.running.len(), 1);
    assert_eq!(list.running[0].instance_id, started.instance_id);
    apps.stop_app(codeos_ipc_client::StopAppRequest {
        instance_id: started.instance_id.clone(),
    });
    let list = apps.list_running().expect("list after stop");
    assert!(list.running.is_empty());
}

#[test]
fn pkg_install_get_uninstall() {
    let apps = std::env::temp_dir().join(format!(
        "codeos-ipc-pkg-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&apps);
    std::fs::create_dir_all(&apps).expect("create apps dir");
    std::env::set_var("CODEOS_APPS_DIR", &apps);

    let fixture_dir = apps.join("fixtures");
    std::fs::create_dir_all(&fixture_dir).expect("fixture dir");
    let manifest = codesvc_pkg::CodeOsManifest {
        app: codesvc_pkg::AppSection {
            id: "com.test.demo".into(),
            name: "Demo".into(),
            version: "1.0.0".into(),
            min_os_version: "0.1.0".into(),
        },
        entry: codesvc_pkg::EntrySection {
            binary: "bin/demo".into(),
            args: vec![],
        },
        ui: None,
        permissions: Default::default(),
        metadata: None,
    };
    let capp_path = fixture_dir.join("demo.capp");
    codesvc_pkg::CappPackager::pack(
        &manifest,
        &capp_path,
        &[("bin/demo", b"stub binary")],
    )
    .expect("pack demo");

    let client = setup_services();
    let pkg = PackageServiceClient::new(client);
    let installed = pkg
        .install(InstallRequest {
            capp_path: capp_path.to_string_lossy().into_owned(),
        })
        .expect("install");
    let info = pkg
        .get_app_info(GetAppInfoRequest {
            app_id: installed.app_id.clone(),
        })
        .expect("info");
    assert_eq!(info.version, installed.version);
    assert_eq!(info.name, "Demo");
    pkg.uninstall(codeos_ipc_client::UninstallRequest {
        app_id: installed.app_id,
    });
    let _ = std::fs::remove_dir_all(&apps);
}

#[test]
fn notif_post_list_clear() {
    let client = setup_services();
    let notif = NotificationServiceClient::new(client);
    let posted = notif
        .post(PostNotificationRequest {
            app_id: "com.test.app".into(),
            title: "Hello".into(),
            body: "World".into(),
            timestamp: 1_700_000_000,
        })
        .expect("post");
    let list = notif.list().expect("list");
    assert_eq!(list.notifications.len(), 1);
    notif.clear(codeos_ipc_client::ClearNotificationRequest {
        notif_id: posted.notif_id,
    });
}

#[test]
fn auth_request_and_check_permission() {
    let client = setup_services();
    let auth = AuthServiceClient::new(client);
    let resp = auth
        .request_permission(RequestPermissionRequest {
            app_id: "com.test.app".into(),
            permission: "storage.read".into(),
        })
        .expect("request");
    assert!(resp.granted);
    let check = auth
        .check_permission(CheckPermissionRequest {
            app_id: "com.test.app".into(),
            permission: "storage.read".into(),
        })
        .expect("check");
    assert!(check.granted);
}

#[test]
fn storage_write_read_list() {
    let client = setup_services();
    let storage = StorageServiceClient::new(client);
    storage.write_file(WriteFileRequest {
        app_id: "com.test.app".into(),
        path: "notes.txt".into(),
        data: "aGVsbG8=".into(),
    });
    let read = storage
        .read_file(ReadFileRequest {
            app_id: "com.test.app".into(),
            path: "notes.txt".into(),
        })
        .expect("read");
    assert_eq!(read.data, "aGVsbG8=");
    let list = storage
        .list_files(ListFilesRequest {
            app_id: "com.test.app".into(),
            path: "".into(),
        })
        .expect("list");
    assert!(list.files.contains(&"notes.txt".to_string()));
}

#[test]
fn invalid_payload_returns_error() {
    let client = setup_services();
    let payload = client
        .request(
            stubs::app::SERVICE_ID,
            stubs::app::methods::START_APP,
            serde_json::json!({}),
        )
        .expect("response payload");
    assert_eq!(
        payload["error"]["code"].as_str().unwrap(),
        error_codes::INVALID_PAYLOAD
    );
}

#[test]
fn unknown_method_returns_not_found() {
    let client = setup_services();
    let payload = client
        .request(
            stubs::window::SERVICE_ID,
            "Window.NoSuchMethod",
            serde_json::json!({}),
        )
        .expect("response payload");
    assert_eq!(
        payload["error"]["code"].as_str().unwrap(),
        error_codes::NOT_FOUND
    );
}

#[test]
fn app_state_changed_event_flow() {
    let (client, app_id) = setup_services_with_installed_app();
    let sub = EventSubscription::subscribe(stubs::app::events::APP_STATE_CHANGED);
    let apps = AppManagerClient::new(client);
    let started = apps
        .start_app(StartAppRequest { app_id })
        .expect("start");
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(1);
    loop {
        let event = sub
            .recv_timeout(std::time::Duration::from_millis(50))
            .expect("event");
        if event.payload["instance_id"] == started.instance_id {
            assert_eq!(event.method, stubs::app::events::APP_STATE_CHANGED);
            assert_eq!(event.payload["state"], "foreground");
            return;
        }
        assert!(
            std::time::Instant::now() < deadline,
            "timed out waiting for matching AppStateChanged event"
        );
    }
}

#[test]
fn unknown_endpoint_returns_bus_error() {
    init_core();
    let client = IpcClient::new("test.client");
    let err = client
        .request("codesvc.missing", "Any.Method", serde_json::json!({}))
        .unwrap_err();
    assert!(matches!(err, IpcError::ServiceError(_)));
}

#[test]
fn multiple_service_clients_from_cloned_ipc_client() {
    let (client, app_id) = setup_services_with_installed_app();
    let appmgr = AppManagerClient::new(client.clone());
    let storage = StorageServiceClient::new(client);
    let started = appmgr
        .start_app(StartAppRequest { app_id: app_id.clone() })
        .expect("start");
    assert!(!started.instance_id.is_empty());
    storage.write_file(WriteFileRequest {
        app_id: "com.test.app".into(),
        path: "shared.txt".into(),
        data: "dGVzdA==".into(),
    });
}
