//! CodeCore — boot orchestration, IPC bus, and service registry.

pub mod boot;
pub mod ipc;
pub mod logging;
pub mod services;

/// Initialize CodeCore subsystems. Called by codeos-init / CodeSim bootstrap.
pub fn init_core() {
    logging::init_logging();
    services::registry::init_registry();
    ipc::bus::init_ipc_bus();
    // TODO: spawn system services and launch System UI.
}
