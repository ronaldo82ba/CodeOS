//! IPC client — connect to codesvc.* endpoints via the CodeCore bus.

mod app;
mod auth;
mod client;
mod error;
mod events;
mod notif;
mod pkg;
pub mod stubs;
mod storage;
mod window;

pub use app::{
    AppManagerClient, AppManagerService, ListRunningAppsResponse, RunningAppInfo, StartAppRequest,
    StartAppResponse, StopAppRequest,
};
pub use auth::{
    AuthService, AuthServiceClient, CheckPermissionRequest, CheckPermissionResponse,
    RequestPermissionRequest, RequestPermissionResponse,
};
pub use client::{IpcClient, IpcError};
pub use error::IpcClientError;
pub use events::{is_event, EventSubscription};
pub use notif::{
    ClearNotificationRequest, ListNotificationsResponse, NotificationInfo,
    NotificationService, NotificationServiceClient, PostNotificationRequest,
    PostNotificationResponse,
};
pub use pkg::{
    AppInfo, GetAppInfoRequest, InstallRequest, InstallResponse, PackageService,
    PackageServiceClient, UninstallRequest,
};
pub use storage::{
    ListFilesRequest, ListFilesResponse, ReadFileRequest, ReadFileResponse, StorageService,
    StorageServiceClient, WriteFileRequest,
};
pub use window::{
    CreateSurfaceRequest, CreateSurfaceResponse, DestroySurfaceRequest, SubmitFrameRequest,
    WindowService, WindowServiceClient,
};
