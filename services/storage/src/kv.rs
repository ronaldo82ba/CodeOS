use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use codecore::ipc::{
    broadcast_event, error_codes, error_response, ok, require_str, response, IpcBus, IpcMessage,
    IpcMessageKind,
};
use codecore::services::types::names;
use serde_json::json;

pub struct StorageService {
    files: Mutex<HashMap<(String, String), Vec<u8>>>,
}

impl StorageService {
    pub fn new() -> Self {
        Self {
            files: Mutex::new(HashMap::new()),
        }
    }

    pub fn handle(&self, msg: &IpcMessage) -> IpcMessage {
        match msg.method.as_str() {
            "Storage.WriteFile" => self.write_file(msg),
            "Storage.ReadFile" => self.read_file(msg),
            "Storage.ListFiles" => self.list_files(msg),
            _ => error_response(
                msg,
                error_codes::NOT_FOUND,
                format!("unknown method: {}", msg.method),
            ),
        }
    }

    fn write_file(&self, msg: &IpcMessage) -> IpcMessage {
        let app_id = match require_str(&msg.payload, "app_id") {
            Ok(v) => v.to_string(),
            Err(e) => return error_response(msg, error_codes::INVALID_PAYLOAD, e),
        };
        let path = match require_str(&msg.payload, "path") {
            Ok(v) => v.to_string(),
            Err(e) => return error_response(msg, error_codes::INVALID_PAYLOAD, e),
        };
        if path.contains("..") || path.starts_with('/') {
            return error_response(msg, error_codes::STORAGE_INVALID_PATH, "invalid path");
        }
        let data_b64 = match require_str(&msg.payload, "data") {
            Ok(v) => v,
            Err(e) => return error_response(msg, error_codes::INVALID_PAYLOAD, e),
        };
        let data = match BASE64.decode(data_b64) {
            Ok(v) => v,
            Err(e) => {
                return error_response(
                    msg,
                    error_codes::INVALID_PAYLOAD,
                    format!("invalid base64 data: {e}"),
                );
            }
        };

        self.files
            .lock()
            .unwrap()
            .insert((app_id.clone(), path.clone()), data);

        broadcast_event(
            names::STORAGE,
            "Storage.FileChanged",
            json!({ "app_id": app_id, "path": path }),
        );

        response(msg, ok(json!({})))
    }

    fn read_file(&self, msg: &IpcMessage) -> IpcMessage {
        let app_id = match require_str(&msg.payload, "app_id") {
            Ok(v) => v.to_string(),
            Err(e) => return error_response(msg, error_codes::INVALID_PAYLOAD, e),
        };
        let path = match require_str(&msg.payload, "path") {
            Ok(v) => v.to_string(),
            Err(e) => return error_response(msg, error_codes::INVALID_PAYLOAD, e),
        };

        let files = self.files.lock().unwrap();
        let Some(data) = files.get(&(app_id, path.clone())) else {
            return error_response(
                msg,
                error_codes::STORAGE_FILE_NOT_FOUND,
                format!("file not found: {path}"),
            );
        };

        response(msg, ok(json!({ "data": BASE64.encode(data) })))
    }

    fn list_files(&self, msg: &IpcMessage) -> IpcMessage {
        let app_id = match require_str(&msg.payload, "app_id") {
            Ok(v) => v.to_string(),
            Err(e) => return error_response(msg, error_codes::INVALID_PAYLOAD, e),
        };
        let prefix = msg
            .payload
            .get("path")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let files: Vec<String> = self
            .files
            .lock()
            .unwrap()
            .keys()
            .filter(|(app, path)| app == &app_id && path.starts_with(prefix))
            .map(|(_, path)| path.clone())
            .collect();

        response(msg, ok(json!({ "files": files })))
    }
}

pub fn register_ipc_endpoint_on(bus: &Arc<Mutex<IpcBus>>) {
    let svc = Arc::new(StorageService::new());
    let svc_in = Arc::clone(&svc);
    bus.lock()
        .expect("ipc bus lock poisoned")
        .register_endpoint(
            names::STORAGE.to_string(),
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
