use std::sync::{Arc, Mutex};

use codecore::ipc::{broadcast_event, error_codes, IpcMessage, IpcMessageKind};
use codecore::services::types::names;
use serde_json::json;

use crate::renderer_hook::get_frame_sink;
use crate::schema::{
    CreateSurfaceRequest, CreateSurfaceResponse, DestroySurfaceRequest, SubmitFrameRequest,
};
use crate::state::WindowState;

pub fn handle_create_surface(
    state: &Arc<Mutex<WindowState>>,
    req: IpcMessage,
) -> Option<IpcMessage> {
    let parsed: CreateSurfaceRequest = match serde_json::from_value(req.payload.clone()) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("[codesvc.window] CreateSurface: invalid payload: {:?}", e);
            return None;
        }
    };

    println!(
        "[codesvc.window] CreateSurface app_id={} size={}x{}",
        parsed.app_id, parsed.width, parsed.height
    );

    let surface_id = {
        let mut st = state.lock().expect("Failed to lock WindowState");
        st.create_surface(parsed.app_id, parsed.width, parsed.height)
    };

    broadcast_event(
        names::WINDOW,
        "Window.SurfaceChanged",
        json!({
            "surface_id": surface_id,
            "width": parsed.width,
            "height": parsed.height,
        }),
    );

    let resp = CreateSurfaceResponse { surface_id };
    let payload = json!(resp);
    Some(IpcMessage::new_response(&req, payload))
}

pub fn handle_destroy_surface(
    state: &Arc<Mutex<WindowState>>,
    req: IpcMessage,
) -> Option<IpcMessage> {
    let parsed: DestroySurfaceRequest = match serde_json::from_value(req.payload.clone()) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("[codesvc.window] DestroySurface: invalid payload: {:?}", e);
            return None;
        }
    };

    println!(
        "[codesvc.window] DestroySurface surface_id={}",
        parsed.surface_id
    );

    {
        let mut st = state.lock().expect("Failed to lock WindowState");
        st.destroy_surface(&parsed.surface_id);
    }

    let payload = json!({});
    Some(IpcMessage::new_response(&req, payload))
}

pub fn handle_submit_frame(
    state: &Arc<Mutex<WindowState>>,
    req: IpcMessage,
) -> Option<IpcMessage> {
    let parsed: SubmitFrameRequest = match serde_json::from_value(req.payload.clone()) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("[codesvc.window] SubmitFrame: invalid payload: {:?}", e);
            return None;
        }
    };

    let surface_info = {
        let st = state.lock().expect("Failed to lock WindowState");
        st.get_surface(&parsed.surface_id)
            .map(|s| (s.app_id.clone(), s.width, s.height))
    };

    if surface_info.is_none() {
        eprintln!(
            "[codesvc.window] SubmitFrame: unknown surface_id={}",
            parsed.surface_id
        );
        return None;
    }

    let (app_id, width, height) = surface_info.unwrap();
    get_frame_sink().submit_frame(
        &parsed.surface_id,
        &app_id,
        width,
        height,
        &parsed.frame_data,
    );

    None
}

pub fn dispatch(state: &Arc<Mutex<WindowState>>, msg: IpcMessage) -> Option<IpcMessage> {
    match msg.method.as_str() {
        "Window.CreateSurface" => handle_create_surface(state, msg),
        "Window.DestroySurface" => handle_destroy_surface(state, msg),
        "Window.SubmitFrame" => handle_submit_frame(state, msg),
        _ => {
            eprintln!("[codesvc.window] Unknown method: {}", msg.method);
            if msg.kind == IpcMessageKind::Request {
                Some(IpcMessage::new_response(
                    &msg,
                    json!({
                        "error": {
                            "code": error_codes::NOT_FOUND,
                            "message": format!("unknown method: {}", msg.method)
                        }
                    }),
                ))
            } else {
                None
            }
        }
    }
}
