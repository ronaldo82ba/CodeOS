use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::IpcClient;

pub const SERVICE_ID: &str = "codesvc.storage";

// ---------------- Storage.WriteFile ----------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WriteFileRequest {
    pub app_id: String,
    pub path: String,
    pub data: String,
}

// ---------------- Storage.ReadFile ----------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReadFileRequest {
    pub app_id: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReadFileResponse {
    pub data: String,
}

// ---------------- Storage.ListFiles ----------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListFilesRequest {
    pub app_id: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListFilesResponse {
    pub files: Vec<String>,
}

// ---------------- Trait ----------------

pub trait StorageService {
    fn write_file(&self, req: WriteFileRequest);
    fn read_file(&self, req: ReadFileRequest) -> Option<ReadFileResponse>;
    fn list_files(&self, req: ListFilesRequest) -> Option<ListFilesResponse>;
}

// ---------------- Implementation ----------------

pub struct StorageServiceClient {
    client: IpcClient,
}

impl StorageServiceClient {
    pub fn new(client: IpcClient) -> Self {
        Self { client }
    }
}

impl StorageService for StorageServiceClient {
    fn write_file(&self, req: WriteFileRequest) {
        let _ = self
            .client
            .request(SERVICE_ID, "Storage.WriteFile", json!(req));
    }

    fn read_file(&self, req: ReadFileRequest) -> Option<ReadFileResponse> {
        let resp = self
            .client
            .request(SERVICE_ID, "Storage.ReadFile", json!(req))
            .ok()?;

        serde_json::from_value(resp).ok()
    }

    fn list_files(&self, req: ListFilesRequest) -> Option<ListFilesResponse> {
        let resp = self
            .client
            .request(SERVICE_ID, "Storage.ListFiles", json!(req))
            .ok()?;

        serde_json::from_value(resp).ok()
    }
}
