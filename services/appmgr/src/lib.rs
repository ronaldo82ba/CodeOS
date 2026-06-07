mod handler;
mod lifecycle;
mod registry;

pub use handler::{
    register_ipc_endpoint, register_ipc_endpoint_on, AppManagerState, ListRunningAppsResponse,
    RunningAppInfo, StartAppRequest, StartAppResponse, StopAppRequest,
};
pub use lifecycle::{AppState, LifecycleManager};
pub use registry::AppRegistry;
