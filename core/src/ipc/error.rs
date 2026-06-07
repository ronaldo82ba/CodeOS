//! Standard IPC error codes for CodeOS v0.1.

pub mod codes {
    pub const NOT_FOUND: &str = "IPC_NOT_FOUND";
    pub const INVALID_PAYLOAD: &str = "IPC_INVALID_PAYLOAD";
    pub const PERMISSION_DENIED: &str = "IPC_PERMISSION_DENIED";
    pub const INTERNAL_ERROR: &str = "IPC_INTERNAL_ERROR";

    // codesvc.window
    pub const WINDOW_SURFACE_NOT_FOUND: &str = "WINDOW_SURFACE_NOT_FOUND";

    // codesvc.app
    pub const APP_NOT_FOUND: &str = "APP_NOT_FOUND";
    pub const APP_ALREADY_RUNNING: &str = "APP_ALREADY_RUNNING";

    // codesvc.pkg
    pub const PKG_APP_NOT_FOUND: &str = "PKG_APP_NOT_FOUND";
    pub const PKG_ALREADY_INSTALLED: &str = "PKG_ALREADY_INSTALLED";
    pub const PKG_INVALID_CAPP: &str = "PKG_INVALID_CAPP";

    // codesvc.notif
    pub const NOTIF_NOT_FOUND: &str = "NOTIF_NOT_FOUND";

    // codesvc.storage
    pub const STORAGE_FILE_NOT_FOUND: &str = "STORAGE_FILE_NOT_FOUND";
    pub const STORAGE_INVALID_PATH: &str = "STORAGE_INVALID_PATH";
}
