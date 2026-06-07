use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::IpcClient;

pub const SERVICE_ID: &str = "codesvc.app";

// ---------------- AppManager.StartApp ----------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StartAppRequest {
    pub app_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StartAppResponse {
    pub instance_id: String,
}

// ---------------- AppManager.StopApp ----------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StopAppRequest {
    pub instance_id: String,
}

// ---------------- AppManager.ListRunningApps ----------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RunningAppInfo {
    pub app_id: String,
    pub instance_id: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListRunningAppsResponse {
    pub running: Vec<RunningAppInfo>,
}

// ---------------- Trait ----------------

pub trait AppManagerService {
    fn start_app(&self, req: StartAppRequest) -> Option<StartAppResponse>;
    fn stop_app(&self, req: StopAppRequest);
    fn list_running(&self) -> Option<ListRunningAppsResponse>;
}

// ---------------- Implementation ----------------

pub struct AppManagerClient {
    client: IpcClient,
}

impl AppManagerClient {
    pub fn new(client: IpcClient) -> Self {
        Self { client }
    }
}

impl AppManagerService for AppManagerClient {
    fn start_app(&self, req: StartAppRequest) -> Option<StartAppResponse> {
        let resp = self
            .client
            .request(SERVICE_ID, "AppManager.StartApp", json!(req))
            .ok()?;

        serde_json::from_value(resp).ok()
    }

    fn stop_app(&self, req: StopAppRequest) {
        let _ = self
            .client
            .request(SERVICE_ID, "AppManager.StopApp", json!(req));
    }

    fn list_running(&self) -> Option<ListRunningAppsResponse> {
        let resp = self
            .client
            .request(SERVICE_ID, "AppManager.ListRunningApps", json!({}))
            .ok()?;

        serde_json::from_value(resp).ok()
    }
}
