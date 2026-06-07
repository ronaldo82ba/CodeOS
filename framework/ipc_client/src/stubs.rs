//! Typed endpoint and method constants for codesvc.* services (IPC Schemas v0.1).

pub mod window {
    pub const SERVICE_ID: &str = "codesvc.window";

    pub mod methods {
        pub const CREATE_SURFACE: &str = "Window.CreateSurface";
        pub const DESTROY_SURFACE: &str = "Window.DestroySurface";
        pub const SUBMIT_FRAME: &str = "Window.SubmitFrame";
    }

    pub mod events {
        pub const SURFACE_CHANGED: &str = "Window.SurfaceChanged";
    }
}

pub mod app {
    pub const SERVICE_ID: &str = "codesvc.app";

    pub mod methods {
        pub const START_APP: &str = "AppManager.StartApp";
        pub const STOP_APP: &str = "AppManager.StopApp";
        pub const LIST_RUNNING_APPS: &str = "AppManager.ListRunningApps";
    }

    pub mod events {
        pub const APP_STATE_CHANGED: &str = "AppManager.AppStateChanged";
    }
}

pub mod pkg {
    pub const SERVICE_ID: &str = "codesvc.pkg";

    pub mod methods {
        pub const INSTALL: &str = "Pkg.Install";
        pub const UNINSTALL: &str = "Pkg.Uninstall";
        pub const GET_APP_INFO: &str = "Pkg.GetAppInfo";
    }

    pub mod events {
        pub const APP_INSTALLED: &str = "Pkg.AppInstalled";
        pub const APP_UNINSTALLED: &str = "Pkg.AppUninstalled";
    }
}

pub mod notif {
    pub const SERVICE_ID: &str = "codesvc.notif";

    pub mod methods {
        pub const POST: &str = "Notif.Post";
        pub const CLEAR: &str = "Notif.Clear";
        pub const LIST: &str = "Notif.List";
    }

    pub mod events {
        pub const NEW_NOTIFICATION: &str = "Notif.NewNotification";
        pub const NOTIFICATION_CLEARED: &str = "Notif.NotificationCleared";
    }
}

pub mod auth {
    pub const SERVICE_ID: &str = "codesvc.auth";

    pub mod methods {
        pub const REQUEST_PERMISSION: &str = "Auth.RequestPermission";
        pub const CHECK_PERMISSION: &str = "Auth.CheckPermission";
    }

    pub mod events {
        pub const PERMISSION_CHANGED: &str = "Auth.PermissionChanged";
    }
}

pub mod storage {
    pub const SERVICE_ID: &str = "codesvc.storage";

    pub mod methods {
        pub const WRITE_FILE: &str = "Storage.WriteFile";
        pub const READ_FILE: &str = "Storage.ReadFile";
        pub const LIST_FILES: &str = "Storage.ListFiles";
    }

    pub mod events {
        pub const FILE_CHANGED: &str = "Storage.FileChanged";
    }
}
