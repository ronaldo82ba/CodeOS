use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use codecore::ipc::{
    broadcast_event, error_codes, error_response, ok, require_str, require_u64, response, IpcBus,
    IpcMessage, IpcMessageKind,
};
use codecore::services::types::names;
use serde_json::json;
use uuid::Uuid;

#[derive(Debug, Clone)]
struct Notification {
    notif_id: String,
    app_id: String,
    title: String,
    body: String,
    timestamp: u64,
}

pub struct NotifService {
    store: Mutex<HashMap<String, Notification>>,
}

impl NotifService {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }

    pub fn handle(&self, msg: &IpcMessage) -> IpcMessage {
        match msg.method.as_str() {
            "Notif.Post" => self.post(msg),
            "Notif.Clear" => self.clear(msg),
            "Notif.List" => self.list(msg),
            _ => error_response(
                msg,
                error_codes::NOT_FOUND,
                format!("unknown method: {}", msg.method),
            ),
        }
    }

    fn post(&self, msg: &IpcMessage) -> IpcMessage {
        let app_id = match require_str(&msg.payload, "app_id") {
            Ok(v) => v.to_string(),
            Err(e) => return error_response(msg, error_codes::INVALID_PAYLOAD, e),
        };
        let title = match require_str(&msg.payload, "title") {
            Ok(v) => v.to_string(),
            Err(e) => return error_response(msg, error_codes::INVALID_PAYLOAD, e),
        };
        let body = match require_str(&msg.payload, "body") {
            Ok(v) => v.to_string(),
            Err(e) => return error_response(msg, error_codes::INVALID_PAYLOAD, e),
        };
        let timestamp = match require_u64(&msg.payload, "timestamp") {
            Ok(v) => v,
            Err(e) => return error_response(msg, error_codes::INVALID_PAYLOAD, e),
        };

        let notif_id = Uuid::new_v4().to_string();
        self.store.lock().unwrap().insert(
            notif_id.clone(),
            Notification {
                notif_id: notif_id.clone(),
                app_id,
                title,
                body,
                timestamp,
            },
        );

        broadcast_event(
            names::NOTIF,
            "Notif.NewNotification",
            json!({ "notif_id": notif_id.clone() }),
        );

        response(msg, ok(json!({ "notif_id": notif_id })))
    }

    fn clear(&self, msg: &IpcMessage) -> IpcMessage {
        let notif_id = match require_str(&msg.payload, "notif_id") {
            Ok(v) => v.to_string(),
            Err(e) => return error_response(msg, error_codes::INVALID_PAYLOAD, e),
        };

        if self.store.lock().unwrap().remove(&notif_id).is_none() {
            return error_response(
                msg,
                error_codes::NOTIF_NOT_FOUND,
                format!("notification not found: {notif_id}"),
            );
        }

        broadcast_event(
            names::NOTIF,
            "Notif.NotificationCleared",
            json!({ "notif_id": notif_id }),
        );

        response(msg, ok(json!({})))
    }

    fn list(&self, msg: &IpcMessage) -> IpcMessage {
        let notifications: Vec<_> = self
            .store
            .lock()
            .unwrap()
            .values()
            .map(|n| {
                json!({
                    "notif_id": n.notif_id,
                    "app_id": n.app_id,
                    "title": n.title,
                    "body": n.body,
                    "timestamp": n.timestamp
                })
            })
            .collect();
        response(msg, ok(json!({ "notifications": notifications })))
    }
}

pub fn register_ipc_endpoint_on(bus: &Arc<Mutex<IpcBus>>) {
    let svc = Arc::new(NotifService::new());
    let svc_in = Arc::clone(&svc);
    bus.lock()
        .expect("ipc bus lock poisoned")
        .register_endpoint(
            names::NOTIF.to_string(),
            Box::new(move |msg| {
                if msg.kind == IpcMessageKind::Request {
                    Some(svc_in.handle(&msg))
                } else {
                    None
                }
            }),
        );
}

pub fn register_ipc_endpoint() {
    register_ipc_endpoint_on(&codecore::ipc::get_global_bus());
}
