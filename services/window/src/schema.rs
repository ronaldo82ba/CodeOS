use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateSurfaceRequest {
    pub app_id: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateSurfaceResponse {
    pub surface_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DestroySurfaceRequest {
    pub surface_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubmitFrameRequest {
    pub surface_id: String,
    pub frame_data: String, // base64
}
