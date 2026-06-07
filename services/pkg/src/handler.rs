use codecore::ipc::{

    broadcast_event, error_codes, error_response, ok, require_str, response, IpcBus, IpcMessage,

    IpcMessageKind,

};

use codecore::services::types::names;

use serde_json::json;



use crate::installer::{CappInstaller, InstallError};

use crate::registry::get_registry;

use crate::uninstaller::{CappUninstaller, UninstallError};

use crate::validator::ValidationError;



pub struct PkgService;



impl PkgService {

    pub fn new() -> Self {

        Self

    }



    pub fn handle(&self, msg: &IpcMessage) -> IpcMessage {

        match msg.method.as_str() {

            "Pkg.Install" => self.install(msg),

            "Pkg.Uninstall" => self.uninstall(msg),

            "Pkg.GetAppInfo" => self.get_app_info(msg),

            _ => error_response(

                msg,

                error_codes::NOT_FOUND,

                format!("unknown method: {}", msg.method),

            ),

        }

    }



    fn install(&self, msg: &IpcMessage) -> IpcMessage {

        let capp_path = match require_str(&msg.payload, "capp_path") {

            Ok(v) => v.to_string(),

            Err(e) => return error_response(msg, error_codes::INVALID_PAYLOAD, e),

        };



        if capp_path.is_empty() {

            return error_response(msg, error_codes::PKG_INVALID_CAPP, "capp_path is empty");

        }



        match CappInstaller::install(std::path::Path::new(&capp_path)) {

            Ok(result) => {

                broadcast_event(

                    names::PKG,

                    "Pkg.AppInstalled",

                    json!({ "app_id": result.app_id }),

                );

                response(

                    msg,

                    ok(json!({

                        "app_id": result.app_id,

                        "version": result.version

                    })),

                )

            }

            Err(InstallError::AlreadyInstalled(app_id)) => error_response(

                msg,

                error_codes::PKG_ALREADY_INSTALLED,

                format!("app already installed: {app_id}"),

            ),

            Err(InstallError::Validation(ValidationError::MissingManifest)) => error_response(

                msg,

                error_codes::PKG_INVALID_CAPP,

                "missing codeos_manifest.toml",

            ),

            Err(InstallError::Validation(ValidationError::MissingBinary(path))) => error_response(

                msg,

                error_codes::PKG_INVALID_CAPP,

                format!("missing entry binary: {path}"),

            ),

            Err(InstallError::Validation(e)) => error_response(

                msg,

                error_codes::PKG_INVALID_CAPP,

                format!("invalid .capp package: {e}"),

            ),

            Err(InstallError::Io(e)) => error_response(

                msg,

                error_codes::PKG_INVALID_CAPP,

                format!("failed to install package: {e}"),

            ),

            Err(InstallError::Zip(e)) => error_response(

                msg,

                error_codes::PKG_INVALID_CAPP,

                format!("failed to install package: {e}"),

            ),

        }

    }



    fn uninstall(&self, msg: &IpcMessage) -> IpcMessage {

        let app_id = match require_str(&msg.payload, "app_id") {

            Ok(v) => v.to_string(),

            Err(e) => return error_response(msg, error_codes::INVALID_PAYLOAD, e),

        };



        match CappUninstaller::uninstall(&app_id) {

            Ok(_) => {

                broadcast_event(names::PKG, "Pkg.AppUninstalled", json!({ "app_id": app_id }));

                response(msg, ok(json!({})))

            }

            Err(UninstallError::NotFound(_)) => error_response(

                msg,

                error_codes::PKG_APP_NOT_FOUND,

                format!("app not found: {app_id}"),

            ),

            Err(UninstallError::Io(e)) => error_response(

                msg,

                error_codes::INTERNAL_ERROR,

                format!("failed to uninstall app: {e}"),

            ),

        }

    }



    fn get_app_info(&self, msg: &IpcMessage) -> IpcMessage {

        let app_id = match require_str(&msg.payload, "app_id") {

            Ok(v) => v.to_string(),

            Err(e) => return error_response(msg, error_codes::INVALID_PAYLOAD, e),

        };



        let registry = get_registry();

        let reg = registry.lock().expect("pkg registry lock poisoned");

        let Some(app) = reg.get(&app_id) else {

            return error_response(

                msg,

                error_codes::PKG_APP_NOT_FOUND,

                format!("app not found: {app_id}"),

            );

        };



        response(

            msg,

            ok(json!({

                "app_id": app.app_id,

                "name": app.name,

                "version": app.version,

                "permissions": app.granted_permission_keys()

            })),

        )

    }

}



pub fn register_ipc_endpoint_on(bus: &std::sync::Arc<std::sync::Mutex<IpcBus>>) {

    let svc = std::sync::Arc::new(PkgService::new());

    let svc_in = std::sync::Arc::clone(&svc);

    bus.lock()

        .expect("ipc bus lock poisoned")

        .register_endpoint(

            names::PKG.to_string(),

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


