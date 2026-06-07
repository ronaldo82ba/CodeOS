//! Message-based IPC — in-process bus in v0.1 (no shared memory).

pub mod bus;
pub mod error;
pub mod events;
pub mod message;
pub mod response;

pub use bus::{
    get_global_bus, init_ipc_bus, shared_bus, IpcBus, IpcBusError, IpcHandler,
};
pub use error::codes as error_codes;
pub use events::{broadcast_event, reset_event_subscribers, subscribe_event};
pub use message::{IpcMessage, IpcMessageKind};
pub use response::{
    err, error_code, error_message, error_response, is_error, ok, require_bool, require_str,
    require_u64, response,
};
