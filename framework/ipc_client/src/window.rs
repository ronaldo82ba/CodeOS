use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::IpcClient;

pub const SERVICE_ID: &str = "codesvc.window";

// ---------------- Window.CreateSurface ----------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateSurfaceRequest {
    pub app_id: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateSurfaceResponse {
    pub surface_id: String,
}

// ---------------- Window.DestroySurface ----------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DestroySurfaceRequest {
    pub surface_id: String,
}

// ---------------- Window.SubmitFrame ----------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SubmitFrameRequest {
    pub surface_id: String,
    pub frame_data: String,
}

// ---------------- Trait ----------------

pub trait WindowService {
    fn create_surface(&self, req: CreateSurfaceRequest) -> Option<CreateSurfaceResponse>;
    fn destroy_surface(&self, req: DestroySurfaceRequest);
    fn submit_frame(&self, req: SubmitFrameRequest);
}

// ---------------- Implementation ----------------

pub struct WindowServiceClient {
    client: IpcClient,
}

impl WindowServiceClient {
    pub fn new(client: IpcClient) -> Self {
        Self { client }
    }
}

impl WindowService for WindowServiceClient {
    fn create_surface(&self, req: CreateSurfaceRequest) -> Option<CreateSurfaceResponse> {
        let resp = self
            .client
            .request(SERVICE_ID, "Window.CreateSurface", json!(req))
            .ok()?;

        serde_json::from_value(resp).ok()
    }

    fn destroy_surface(&self, req: DestroySurfaceRequest) {
        let _ = self
            .client
            .request(SERVICE_ID, "Window.DestroySurface", json!(req));
    }

    fn submit_frame(&self, req: SubmitFrameRequest) {
        self.client
            .event(SERVICE_ID, "Window.SubmitFrame", json!(req));
    }
}
