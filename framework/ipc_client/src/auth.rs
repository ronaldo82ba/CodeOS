use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::IpcClient;

pub const SERVICE_ID: &str = "codesvc.auth";

// ---------------- Auth.RequestPermission ----------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RequestPermissionRequest {
    pub app_id: String,
    pub permission: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RequestPermissionResponse {
    pub granted: bool,
}

// ---------------- Auth.CheckPermission ----------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CheckPermissionRequest {
    pub app_id: String,
    pub permission: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CheckPermissionResponse {
    pub granted: bool,
}

// ---------------- Trait ----------------

pub trait AuthService {
    fn request_permission(
        &self,
        req: RequestPermissionRequest,
    ) -> Option<RequestPermissionResponse>;
    fn check_permission(&self, req: CheckPermissionRequest) -> Option<CheckPermissionResponse>;
}

// ---------------- Implementation ----------------

pub struct AuthServiceClient {
    client: IpcClient,
}

impl AuthServiceClient {
    pub fn new(client: IpcClient) -> Self {
        Self { client }
    }
}

impl AuthService for AuthServiceClient {
    fn request_permission(
        &self,
        req: RequestPermissionRequest,
    ) -> Option<RequestPermissionResponse> {
        let resp = self
            .client
            .request(SERVICE_ID, "Auth.RequestPermission", json!(req))
            .ok()?;

        serde_json::from_value(resp).ok()
    }

    fn check_permission(&self, req: CheckPermissionRequest) -> Option<CheckPermissionResponse> {
        let resp = self
            .client
            .request(SERVICE_ID, "Auth.CheckPermission", json!(req))
            .ok()?;

        serde_json::from_value(resp).ok()
    }
}
