use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use codecore::ipc::{
    broadcast_event, error_codes, get_global_bus, IpcBus, IpcMessage, IpcMessageKind,
};
use codecore::services::types::names;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

// ---------------- IPC schema ----------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartAppRequest {
    pub app_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartAppResponse {
    pub instance_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopAppRequest {
    pub instance_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunningAppInfo {
    pub app_id: String,
    pub instance_id: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRunningAppsResponse {
    pub running: Vec<RunningAppInfo>,
}

// ---------------- State ----------------

struct AppInstance {
    app_id: String,
    state: String,
}

pub struct AppManagerState {
    instances: Mutex<HashMap<String, AppInstance>>,
}

impl AppManagerState {
    pub fn new() -> Self {
        Self {
            instances: Mutex::new(HashMap::new()),
        }
    }

    fn handle(&self, msg: &IpcMessage) -> Option<IpcMessage> {
        match msg.method.as_str() {
            "AppManager.StartApp" => Some(self.start_app(msg)),
            "AppManager.StopApp" => Some(self.stop_app(msg)),
            "AppManager.ListRunningApps" => Some(self.list_running(msg)),
            _ => Some(IpcMessage::new_response(
                msg,
                json!({
                    "error": {
                        "code": error_codes::NOT_FOUND,
                        "message": format!("unknown method: {}", msg.method)
                    }
                }),
            )),
        }
    }

    fn start_app(&self, msg: &IpcMessage) -> IpcMessage {
        let req: StartAppRequest = match serde_json::from_value::<StartAppRequest>(msg.payload.clone()) {
            Ok(r) if !r.app_id.is_empty() => r,
            _ => {
                return IpcMessage::new_response(
                    msg,
                    json!({
                        "error": {
                            "code": error_codes::INVALID_PAYLOAD,
                            "message": "missing or invalid field: app_id"
                        }
                    }),
                );
            }
        };

        let Some(installed) = codesvc_pkg::lookup_app(&req.app_id) else {
            return IpcMessage::new_response(
                msg,
                json!({
                    "error": {
                        "code": error_codes::APP_NOT_FOUND,
                        "message": format!("installed app not found: {}", req.app_id)
                    }
                }),
            );
        };

        let binary_path = installed.entry_binary_path();
        let manifest_path = installed.manifest_path();
        let app_data_dir = installed.install_path.clone();
        let _launch_env = [
            ("APP_ID", req.app_id.as_str()),
            ("APP_DATA_DIR", app_data_dir.to_string_lossy().as_ref()),
            ("MANIFEST_PATH", manifest_path.to_string_lossy().as_ref()),
        ];
        let _entry = binary_path;

        let instance_id = format!("instance-{}", Uuid::new_v4());
        self.instances.lock().unwrap().insert(
            instance_id.clone(),
            AppInstance {
                app_id: req.app_id.clone(),
                state: "foreground".to_string(),
            },
        );

        broadcast_event(
            names::APP,
            "AppManager.AppStateChanged",
            json!({ "instance_id": instance_id, "state": "foreground" }),
        );

        IpcMessage::new_response(
            msg,
            serde_json::to_value(StartAppResponse { instance_id }).unwrap(),
        )
    }

    fn stop_app(&self, msg: &IpcMessage) -> IpcMessage {
        let req: StopAppRequest = match serde_json::from_value::<StopAppRequest>(msg.payload.clone()) {
            Ok(r) => r,
            Err(_) => {
                return IpcMessage::new_response(
                    msg,
                    json!({
                        "error": {
                            "code": error_codes::INVALID_PAYLOAD,
                            "message": "missing or invalid field: instance_id"
                        }
                    }),
                );
            }
        };

        if self
            .instances
            .lock()
            .unwrap()
            .remove(&req.instance_id)
            .is_none()
        {
            return IpcMessage::new_response(
                msg,
                json!({
                    "error": {
                        "code": error_codes::APP_NOT_FOUND,
                        "message": format!("instance not found: {}", req.instance_id)
                    }
                }),
            );
        }

        broadcast_event(
            names::APP,
            "AppManager.AppStateChanged",
            json!({ "instance_id": req.instance_id, "state": "stopped" }),
        );

        IpcMessage::new_response(
            msg,
            json!({
                "status": "stopped",
                "instance_id": req.instance_id
            }),
        )
    }

    fn list_running(&self, msg: &IpcMessage) -> IpcMessage {
        let running: Vec<RunningAppInfo> = self
            .instances
            .lock()
            .unwrap()
            .iter()
            .map(|(instance_id, inst)| RunningAppInfo {
                app_id: inst.app_id.clone(),
                instance_id: instance_id.clone(),
                state: inst.state.clone(),
            })
            .collect();

        IpcMessage::new_response(
            msg,
            serde_json::to_value(ListRunningAppsResponse { running }).unwrap(),
        )
    }
}

pub fn register_ipc_endpoint_on(bus: &Arc<Mutex<IpcBus>>) {
    let state = Arc::new(AppManagerState::new());
    let state_in = Arc::clone(&state);
    bus.lock()
        .expect("ipc bus lock poisoned")
        .register_endpoint(
            names::APP.to_string(),
            Box::new(move |msg| {
                if msg.kind == IpcMessageKind::Request {
                    state_in.handle(&msg)
                } else {
                    None
                }
            }),
        );
}

pub fn register_ipc_endpoint() {
    register_ipc_endpoint_on(&get_global_bus());
}
