use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::IpcClient;

pub const SERVICE_ID: &str = "codesvc.notif";

// ---------------- Notif.Post ----------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PostNotificationRequest {
    pub app_id: String,
    pub title: String,
    pub body: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PostNotificationResponse {
    pub notif_id: String,
}

// ---------------- Notif.Clear ----------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClearNotificationRequest {
    pub notif_id: String,
}

// ---------------- Notif.List ----------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NotificationInfo {
    pub notif_id: String,
    pub app_id: String,
    pub title: String,
    pub body: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListNotificationsResponse {
    pub notifications: Vec<NotificationInfo>,
}

// ---------------- Trait ----------------

pub trait NotificationService {
    fn post(&self, req: PostNotificationRequest) -> Option<PostNotificationResponse>;
    fn clear(&self, req: ClearNotificationRequest);
    fn list(&self) -> Option<ListNotificationsResponse>;
}

// ---------------- Implementation ----------------

pub struct NotificationServiceClient {
    client: IpcClient,
}

impl NotificationServiceClient {
    pub fn new(client: IpcClient) -> Self {
        Self { client }
    }
}

impl NotificationService for NotificationServiceClient {
    fn post(&self, req: PostNotificationRequest) -> Option<PostNotificationResponse> {
        let resp = self
            .client
            .request(SERVICE_ID, "Notif.Post", json!(req))
            .ok()?;

        serde_json::from_value(resp).ok()
    }

    fn clear(&self, req: ClearNotificationRequest) {
        let _ = self
            .client
            .request(SERVICE_ID, "Notif.Clear", json!(req));
    }

    fn list(&self) -> Option<ListNotificationsResponse> {
        let resp = self
            .client
            .request(SERVICE_ID, "Notif.List", json!({}))
            .ok()?;

        serde_json::from_value(resp).ok()
    }
}
