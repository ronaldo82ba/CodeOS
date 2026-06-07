use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;
use serde_json::Value;

use crate::ipc::message::{IpcMessage, IpcMessageKind};
use crate::services::registry;

pub type IpcHandler = Box<dyn Fn(IpcMessage) -> Option<IpcMessage> + Send + Sync>;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum IpcBusError {
    #[error("endpoint not found: {0}")]
    EndpointNotFound(String),
    #[error("handler returned no response")]
    NoResponse,
    #[error("handler returned invalid message kind: expected response")]
    InvalidResponse,
}

/// Central in-process message bus — routes messages between endpoints by `to`.
pub struct IpcBus {
    handlers: HashMap<String, IpcHandler>,
}

impl IpcBus {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn register_endpoint(&mut self, id: String, handler: IpcHandler) {
        self.handlers.insert(id.clone(), handler);
        registry::register_endpoint(&id);
    }

    /// Deliver an event to `to`; handler return value is ignored.
    pub fn send_event(&self, from: &str, to: &str, method: &str, payload: Value) {
        let msg = IpcMessage::new_event(from, to, method, payload);
        if let Some(handler) = self.handlers.get(to) {
            let _ = handler(msg);
        }
    }

    /// Synchronous request — handler must return `Some(IpcMessage)` with kind `Response`.
    pub fn send_request(
        &self,
        from: &str,
        to: &str,
        method: &str,
        payload: Value,
    ) -> Result<Value, IpcBusError> {
        let request = IpcMessage::new_request(from, to, method, payload);
        let handler = self
            .handlers
            .get(to)
            .ok_or_else(|| IpcBusError::EndpointNotFound(to.to_string()))?;

        match handler(request) {
            Some(response) if response.kind == IpcMessageKind::Response => Ok(response.payload),
            Some(_) => Err(IpcBusError::InvalidResponse),
            None => Err(IpcBusError::NoResponse),
        }
    }
}

impl Default for IpcBus {
    fn default() -> Self {
        Self::new()
    }
}

static GLOBAL_BUS: Lazy<Arc<Mutex<IpcBus>>> =
    Lazy::new(|| Arc::new(Mutex::new(IpcBus::new())));

pub fn init_ipc_bus() {
    Lazy::force(&GLOBAL_BUS);
    tracing::info!("IPC bus initialized");
}

pub fn get_global_bus() -> Arc<Mutex<IpcBus>> {
    GLOBAL_BUS.clone()
}

/// Alias for [`get_global_bus`] (integration tests / simulator).
pub fn shared_bus() -> Arc<Mutex<IpcBus>> {
    get_global_bus()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn request_response_roundtrip() {
        let mut bus = IpcBus::new();
        bus.register_endpoint(
            "test.echo".into(),
            Box::new(|msg| Some(IpcMessage::new_response(&msg, json!({ "echo": true })))),
        );

        let payload = bus
            .send_request("test.client", "test.echo", "ping", json!({}))
            .expect("roundtrip");
        assert_eq!(payload["echo"], true);
    }

    #[test]
    fn event_is_delivered() {
        use std::sync::Mutex as StdMutex;

        let mut bus = IpcBus::new();
        let seen = Arc::new(StdMutex::new(String::new()));
        let seen_in_handler = Arc::clone(&seen);

        bus.register_endpoint(
            "test.events".into(),
            Box::new(move |msg| {
                if msg.kind == IpcMessageKind::Event {
                    *seen_in_handler.lock().unwrap() = msg.method.clone();
                }
                None
            }),
        );

        bus.send_event("test.client", "test.events", "Test.Event", json!({}));

        assert_eq!(*seen.lock().unwrap(), "Test.Event");
    }

    #[test]
    fn unknown_endpoint_returns_error() {
        let bus = IpcBus::new();
        let err = bus
            .send_request("test.client", "missing.endpoint", "ping", json!({}))
            .unwrap_err();
        assert!(matches!(err, IpcBusError::EndpointNotFound(_)));
    }
}
