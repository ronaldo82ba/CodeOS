mod renderer;
mod window;

use std::sync::{Arc, Mutex};

use codecore::init_core;
use codecore::services::registry::{register_service, set_service_state};
use codecore::services::types::{names, ServiceDescriptor, ServiceState};
use codesvc_window::{set_frame_sink, CallbackFrameSink};
use renderer::SimulatorRenderer;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use window::DeviceWindow;

fn register_system_service(name: &str, register_ipc: fn()) {
    let pid = std::process::id();
    register_service(ServiceDescriptor::new(name, pid)).expect("register service");
    set_service_state(name, ServiceState::Running).ok();
    register_ipc();
    info!(service = name, "registered");
}

fn main() {
    println!("CodeSim (CodeOS Simulator) v0.1 starting...");

    FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .init();

    init_core();

    let device = DeviceWindow::phone_default();
    let renderer = Arc::new(Mutex::new(SimulatorRenderer::new(
        device.width(),
        device.height(),
    )));

    let renderer_for_sink = Arc::clone(&renderer);
    set_frame_sink(Arc::new(CallbackFrameSink::new(
        move |surface_id, _app_id, _width, _height, frame_data| {
            renderer_for_sink
                .lock()
                .expect("renderer lock")
                .present_frame(surface_id, frame_data);
        },
    )));

    register_system_service(names::WINDOW, codesvc_window::register_ipc_endpoint);
    register_system_service(names::APP, codesvc_appmgr::register_ipc_endpoint);
    register_system_service(names::PKG, codesvc_pkg::register_ipc_endpoint);
    register_system_service(names::NOTIF, codesvc_notif::register_ipc_endpoint);
    register_system_service(names::AUTH, codesvc_auth::register_ipc_endpoint);
    register_system_service(names::STORAGE, codesvc_storage::register_ipc_endpoint);

    renderer
        .lock()
        .expect("renderer lock")
        .render_device_screen();

    println!(
        "CodeSim ready — device {}x{}, compositor wired via FrameSink.",
        device.dimensions().0,
        device.dimensions().1
    );
    println!("Use `codeos run --simulator` to launch apps.");
}
