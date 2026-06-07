use codecore::services::registry::{register_service, set_service_state};
use codecore::services::types::{names, ServiceDescriptor, ServiceState};
use codesvc_pkg::register_ipc_endpoint;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .init();

    codecore::init_core();

    let pid = std::process::id();
    register_service(ServiceDescriptor::new(names::PKG, pid)).expect("register");
    set_service_state(names::PKG, ServiceState::Running).ok();

    register_ipc_endpoint();
    info!(endpoint = names::PKG, "codesvc.pkg started");

    tokio::signal::ctrl_c()
        .await
        .expect("failed to listen for ctrl-c");
    set_service_state(names::PKG, ServiceState::Stopped).ok();
    info!("codesvc.pkg shutting down");
}
