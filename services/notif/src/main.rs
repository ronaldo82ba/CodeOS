mod store;

use codecore::services::registry::{register_service, set_service_state};
use codecore::services::types::{names, ServiceDescriptor, ServiceState};
use codesvc_notif::register_ipc_endpoint;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .init();

    codecore::init_core();

    let pid = std::process::id();
    register_service(ServiceDescriptor::new(names::NOTIF, pid)).expect("register");
    set_service_state(names::NOTIF, ServiceState::Running).ok();

    register_ipc_endpoint();
    info!(endpoint = names::NOTIF, "codesvc.notif started");

    tokio::signal::ctrl_c()
        .await
        .expect("failed to listen for ctrl-c");
    set_service_state(names::NOTIF, ServiceState::Stopped).ok();
    info!("codesvc.notif shutting down");
}
