use codecore::services::registry::{register_service, set_service_state};

use codecore::services::types::{names, ServiceDescriptor, ServiceState};

use codesvc_window::register_ipc_endpoint;

use tracing::{info, Level};

use tracing_subscriber::FmtSubscriber;



#[tokio::main]

async fn main() {

    FmtSubscriber::builder()

        .with_max_level(Level::INFO)

        .init();



    codecore::init_core();



    let pid = std::process::id();

    register_service(ServiceDescriptor::new(names::WINDOW, pid)).expect("register");

    set_service_state(names::WINDOW, ServiceState::Running).ok();



    register_ipc_endpoint();

    info!(endpoint = names::WINDOW, "codesvc.window started");



    tokio::signal::ctrl_c()

        .await

        .expect("failed to listen for ctrl-c");

    set_service_state(names::WINDOW, ServiceState::Stopped).ok();

    info!("codesvc.window shutting down");

}


