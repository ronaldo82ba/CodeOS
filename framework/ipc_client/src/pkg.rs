use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::IpcClient;

pub const SERVICE_ID: &str = "codesvc.pkg";

// ---------------- Pkg.Install ----------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InstallRequest {
    pub capp_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InstallResponse {
    pub app_id: String,
    pub version: String,
}

// ---------------- Pkg.Uninstall ----------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UninstallRequest {
    pub app_id: String,
}

// ---------------- Pkg.GetAppInfo ----------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetAppInfoRequest {
    pub app_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppInfo {
    pub app_id: String,
    pub name: String,
    pub version: String,
    pub permissions: Vec<String>,
}

// ---------------- Trait ----------------

pub trait PackageService {
    fn install(&self, req: InstallRequest) -> Option<InstallResponse>;
    fn uninstall(&self, req: UninstallRequest);
    fn get_app_info(&self, req: GetAppInfoRequest) -> Option<AppInfo>;
}

// ---------------- Implementation ----------------

pub struct PackageServiceClient {
    client: IpcClient,
}

impl PackageServiceClient {
    pub fn new(client: IpcClient) -> Self {
        Self { client }
    }
}

impl PackageService for PackageServiceClient {
    fn install(&self, req: InstallRequest) -> Option<InstallResponse> {
        let resp = self
            .client
            .request(SERVICE_ID, "Pkg.Install", json!(req))
            .ok()?;

        serde_json::from_value(resp).ok()
    }

    fn uninstall(&self, req: UninstallRequest) {
        let _ = self
            .client
            .request(SERVICE_ID, "Pkg.Uninstall", json!(req));
    }

    fn get_app_info(&self, req: GetAppInfoRequest) -> Option<AppInfo> {
        let resp = self
            .client
            .request(SERVICE_ID, "Pkg.GetAppInfo", json!(req))
            .ok()?;

        serde_json::from_value(resp).ok()
    }
}
