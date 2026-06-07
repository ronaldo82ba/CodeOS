use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use codecore::init_core;
use codecore::ipc::{error_codes, IpcBus};
use codeos_ipc_client::{
    AuthService, AuthServiceClient, CheckPermissionRequest, GetAppInfoRequest, InstallRequest,
    IpcClient, PackageService, PackageServiceClient, UninstallRequest,
};
use codesvc_auth::register_ipc_endpoint_on;
use codesvc_pkg::{
    register_ipc_endpoint_on as register_pkg, AppSection, CappPackager, CodeOsManifest,
    EntrySection, MetadataSection, PermissionsSection, UiSection,
};
use codesvc_pkg::{apps_dir, get_registry};

static PKG_TEST_COUNTER: AtomicU64 = AtomicU64::new(0);
static PKG_TEST_LOCK: Mutex<()> = Mutex::new(());

fn next_test_id(name: &str) -> String {
    let n = PKG_TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("com.test.{name}.{n}")
}

fn test_apps_dir(test_name: &str) -> PathBuf {
    let n = PKG_TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
    std::env::temp_dir().join(format!(
        "codeos-pkg-test-{test_name}-{}-{n}",
        std::process::id()
    ))
}

fn setup_with_apps_dir(dir: &Path) -> IpcClient {
    let _guard = PKG_TEST_LOCK.lock().expect("pkg test lock");
    let _ = std::fs::remove_dir_all(dir);
    std::fs::create_dir_all(dir).expect("create apps dir");
    std::env::set_var("CODEOS_APPS_DIR", dir);

    init_core();
    codecore::ipc::reset_event_subscribers();
    get_registry()
        .lock()
        .expect("registry lock")
        .clear();

    let bus = Arc::new(Mutex::new(IpcBus::new()));
    register_pkg(&bus);
    register_ipc_endpoint_on(&bus);
    IpcClient::with_bus("test.pkg.client", bus)
}

fn sample_manifest(app_id: &str) -> CodeOsManifest {
    CodeOsManifest {
        app: AppSection {
            id: app_id.into(),
            name: "Integration Test App".into(),
            version: "2.0.0".into(),
            min_os_version: "0.1.0".into(),
        },
        entry: EntrySection {
            binary: "bin/myapp".into(),
            args: vec![],
        },
        ui: Some(UiSection {
            main_view: "res/main_layout.json".into(),
            icon: "assets/icon.png".into(),
        }),
        permissions: PermissionsSection {
            network: true,
            storage: true,
            notifications: false,
        },
        metadata: Some(MetadataSection {
            author: "Ronaldo Mijares".into(),
            website: "https://example.com".into(),
            description: "Integration test fixture".into(),
        }),
    }
}

fn build_fixture_capp(dir: &Path, app_id: &str) -> PathBuf {
    std::fs::create_dir_all(dir).expect("create fixture dir");
    let manifest = sample_manifest(app_id);
    let capp_path = dir.join("test-integration.capp");
    let payload = [
        ("bin/myapp", b"#!/bin/sh\necho hello" as &[u8]),
        ("assets/icon.png", b"PNG" as &[u8]),
        ("res/main_layout.json", br#"{"type":"view"}"# as &[u8]),
    ];
    CappPackager::pack(&manifest, &capp_path, &payload).expect("pack fixture");
    capp_path
}

#[test]
fn pkg_install_validates_and_extracts() {
    let app_id = next_test_id("install");
    let apps = test_apps_dir("install");
    let fixture_dir = apps.join("fixtures");
    let client = setup_with_apps_dir(&apps);
    let capp_path = build_fixture_capp(&fixture_dir, &app_id);

    let pkg = PackageServiceClient::new(client.clone());
    let installed = pkg
        .install(InstallRequest {
            capp_path: capp_path.to_string_lossy().into_owned(),
        })
        .expect("install");

    assert_eq!(installed.app_id, app_id);
    assert_eq!(installed.version, "2.0.0");

    let install_path = apps.join(&app_id);
    assert!(install_path.join("codeos_manifest.toml").is_file());
    assert!(install_path.join("bin/myapp").is_file());
    assert!(install_path.join("assets/icon.png").is_file());
    assert!(install_path.join("res/main_layout.json").is_file());

    pkg.uninstall(UninstallRequest {
        app_id: installed.app_id,
    });
    assert!(!install_path.exists());
    let _ = std::fs::remove_dir_all(&apps);
}

#[test]
fn pkg_get_app_info_returns_manifest_data() {
    let app_id = next_test_id("get-info");
    let apps = test_apps_dir("get-info");
    let fixture_dir = apps.join("fixtures");
    let client = setup_with_apps_dir(&apps);
    let capp_path = build_fixture_capp(&fixture_dir, &app_id);

    let pkg = PackageServiceClient::new(client.clone());
    let installed = pkg
        .install(InstallRequest {
            capp_path: capp_path.to_string_lossy().into_owned(),
        })
        .expect("install");

    let info = pkg
        .get_app_info(GetAppInfoRequest {
            app_id: installed.app_id.clone(),
        })
        .expect("get app info");

    assert_eq!(info.app_id, app_id);
    assert_eq!(info.name, "Integration Test App");
    assert_eq!(info.version, "2.0.0");
    assert!(info.permissions.contains(&"network".to_string()));
    assert!(info.permissions.contains(&"storage".to_string()));
    assert!(!info.permissions.contains(&"notifications".to_string()));

    pkg.uninstall(UninstallRequest {
        app_id: installed.app_id,
    });
    let _ = std::fs::remove_dir_all(&apps);
}

#[test]
fn pkg_uninstall_removes_app() {
    let app_id = next_test_id("uninstall");
    let apps = test_apps_dir("uninstall");
    let fixture_dir = apps.join("fixtures");
    let client = setup_with_apps_dir(&apps);
    let capp_path = build_fixture_capp(&fixture_dir, &app_id);

    let pkg = PackageServiceClient::new(client.clone());
    let installed = pkg
        .install(InstallRequest {
            capp_path: capp_path.to_string_lossy().into_owned(),
        })
        .expect("install");

    pkg.uninstall(UninstallRequest {
        app_id: installed.app_id.clone(),
    });

    assert!(
        pkg.get_app_info(GetAppInfoRequest {
            app_id: installed.app_id,
        })
        .is_none()
    );
    assert!(!apps.join(&app_id).exists());
    let _ = std::fs::remove_dir_all(&apps);
}

#[test]
fn pkg_rejects_invalid_capp() {
    let apps = test_apps_dir("invalid");
    let client = setup_with_apps_dir(&apps);
    let bad_path = apps.join("not-a-capp.txt");
    std::fs::write(&bad_path, "not zip").expect("write bad file");

    let payload = client
        .request(
            "codesvc.pkg",
            "Pkg.Install",
            serde_json::json!({ "capp_path": bad_path.to_string_lossy() }),
        )
        .expect("response payload");

    assert_eq!(
        payload["error"]["code"].as_str().unwrap(),
        error_codes::PKG_INVALID_CAPP
    );
    let _ = std::fs::remove_dir_all(&apps);
}

#[test]
fn pkg_install_grants_auth_permissions() {
    let app_id = next_test_id("auth");
    let apps = test_apps_dir("auth");
    let fixture_dir = apps.join("fixtures");
    let client = setup_with_apps_dir(&apps);
    let capp_path = build_fixture_capp(&fixture_dir, &app_id);

    let pkg = PackageServiceClient::new(client.clone());
    let installed = pkg
        .install(InstallRequest {
            capp_path: capp_path.to_string_lossy().into_owned(),
        })
        .expect("install");

    let auth = AuthServiceClient::new(client);
    let network = auth
        .check_permission(CheckPermissionRequest {
            app_id: installed.app_id.clone(),
            permission: "network".into(),
        })
        .expect("check network");
    assert!(network.granted);

    let notifications = auth
        .check_permission(CheckPermissionRequest {
            app_id: installed.app_id.clone(),
            permission: "notifications".into(),
        })
        .expect("check notifications");
    assert!(!notifications.granted);

    pkg.uninstall(UninstallRequest {
        app_id: installed.app_id,
    });
    let _ = std::fs::remove_dir_all(&apps);
}

#[test]
fn apps_dir_defaults_to_configurable_path() {
    let _guard = PKG_TEST_LOCK.lock().expect("pkg test lock");
    let previous = std::env::var("CODEOS_APPS_DIR").ok();
    std::env::remove_var("CODEOS_APPS_DIR");
    assert_eq!(apps_dir(), PathBuf::from("./data/apps"));
    if let Some(value) = previous {
        std::env::set_var("CODEOS_APPS_DIR", value);
    }
}
