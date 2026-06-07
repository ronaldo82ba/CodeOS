use codecore::ipc::{subscribe_event, IpcMessage, IpcMessageKind};

use crate::error::IpcClientError;
use crate::IpcError;

/// v0.1 event subscription handle — blocks on [`EventSubscription::recv`].
pub struct EventSubscription {
    rx: std::sync::mpsc::Receiver<IpcMessage>,
}

impl EventSubscription {
    pub fn subscribe(event_method: &str) -> Self {
        Self {
            rx: subscribe_event(event_method),
        }
    }

    pub fn recv(&self) -> Result<IpcMessage, IpcClientError> {
        self.rx
            .recv()
            .map_err(|_| IpcClientError::Ipc(IpcError::NoResponse))
    }

    pub fn recv_timeout(
        &self,
        timeout: std::time::Duration,
    ) -> Result<IpcMessage, IpcClientError> {
        self.rx
            .recv_timeout(timeout)
            .map_err(|_| IpcClientError::Ipc(IpcError::NoResponse))
    }

    pub fn method(&self) -> &str {
        "event"
    }
}

pub fn is_event(msg: &IpcMessage) -> bool {
    msg.kind == IpcMessageKind::Event
}
