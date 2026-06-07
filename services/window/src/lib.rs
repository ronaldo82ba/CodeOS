mod handlers;
mod renderer_hook;
mod schema;
mod state;

pub use handlers::{handle_create_surface, handle_destroy_surface, handle_submit_frame};
pub use renderer_hook::{
    set_frame_sink, CallbackFrameSink, FrameSink, LogFrameSink,
};
pub use schema::{
    CreateSurfaceRequest, CreateSurfaceResponse, DestroySurfaceRequest, SubmitFrameRequest,
};
pub use state::{Surface, WindowState};

use std::sync::{Arc, Mutex};

use codecore::ipc::{get_global_bus, IpcBus, IpcMessageKind};

pub fn register_ipc_endpoint_on(bus: &Arc<Mutex<IpcBus>>) {
    let state = Arc::new(Mutex::new(WindowState::default()));
    let state_in = Arc::clone(&state);
    bus.lock()
        .expect("Failed to lock IPC bus")
        .register_endpoint(
            codecore::services::types::names::WINDOW.to_string(),
            Box::new(move |msg| {
                if msg.kind == IpcMessageKind::Request
                    || (msg.kind == IpcMessageKind::Event && msg.method == "Window.SubmitFrame")
                {
                    handlers::dispatch(&state_in, msg)
                } else {
                    None
                }
            }),
        );
}

pub fn register_ipc_endpoint() {
    register_ipc_endpoint_on(&get_global_bus());
}
