use codecore::ipc::{

    broadcast_event, error_codes, error_response, ok, require_str, response, IpcBus, IpcMessage,

    IpcMessageKind,

};

use codecore::services::types::names;

use serde_json::json;



use crate::grants;



pub struct AuthService;



impl AuthService {

    pub fn new() -> Self {

        Self

    }



    pub fn handle(&self, msg: &IpcMessage) -> IpcMessage {

        match msg.method.as_str() {

            "Auth.RequestPermission" => self.request_permission(msg),

            "Auth.CheckPermission" => self.check_permission(msg),

            _ => error_response(

                msg,

                error_codes::NOT_FOUND,

                format!("unknown method: {}", msg.method),

            ),

        }

    }



    fn request_permission(&self, msg: &IpcMessage) -> IpcMessage {

        let (app_id, permission) = match self.parse_app_permission(msg) {

            Ok(v) => v,

            Err(e) => return error_response(msg, error_codes::INVALID_PAYLOAD, e),

        };



        let granted = true;

        grants::grant(&app_id, &permission);



        broadcast_event(

            names::AUTH,

            "Auth.PermissionChanged",

            json!({

                "app_id": app_id,

                "permission": permission,

                "granted": granted

            }),

        );



        response(msg, ok(json!({ "granted": granted })))

    }



    fn check_permission(&self, msg: &IpcMessage) -> IpcMessage {

        let (app_id, permission) = match self.parse_app_permission(msg) {

            Ok(v) => v,

            Err(e) => return error_response(msg, error_codes::INVALID_PAYLOAD, e),

        };



        let granted = grants::is_granted(&app_id, &permission);

        response(msg, ok(json!({ "granted": granted })))

    }



    fn parse_app_permission(&self, msg: &IpcMessage) -> Result<(String, String), String> {

        let app_id = require_str(&msg.payload, "app_id")?.to_string();

        let permission = require_str(&msg.payload, "permission")?.to_string();

        Ok((app_id, permission))

    }

}



pub fn register_ipc_endpoint_on(bus: &std::sync::Arc<std::sync::Mutex<IpcBus>>) {

    let svc = std::sync::Arc::new(AuthService::new());

    let svc_in = std::sync::Arc::clone(&svc);

    bus.lock()

        .expect("ipc bus lock poisoned")

        .register_endpoint(

            names::AUTH.to_string(),

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


