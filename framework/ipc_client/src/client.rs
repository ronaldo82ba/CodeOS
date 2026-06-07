use codecore::ipc::{get_global_bus, IpcBus, IpcBusError, IpcMessageKind};
use serde_json::Value;
use std::sync::{Arc, Mutex};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum IpcError {
    #[error("handler returned no response")]
    NoResponse,
    #[error("service error: {0}")]
    ServiceError(String),
}

impl From<IpcBusError> for IpcError {
    fn from(err: IpcBusError) -> Self {
        match err {
            IpcBusError::NoResponse => IpcError::NoResponse,
            IpcBusError::EndpointNotFound(endpoint) => {
                IpcError::ServiceError(format!("endpoint not found: {endpoint}"))
            }
            IpcBusError::InvalidResponse => {
                IpcError::ServiceError("handler returned invalid response kind".into())
            }
        }
    }
}

#[derive(Clone)]
pub struct IpcClient {
    bus: Arc<Mutex<IpcBus>>,
    from_id: String,
}

impl IpcClient {
    pub fn new(from_id: impl Into<String>) -> Self {
        Self {
            bus: get_global_bus(),
            from_id: from_id.into(),
        }
    }

    pub fn with_bus(from_id: impl Into<String>, bus: Arc<Mutex<IpcBus>>) -> Self {
        Self {
            bus,
            from_id: from_id.into(),
        }
    }

    pub fn from_id(&self) -> &str {
        &self.from_id
    }

    /// Synchronous blocking request — handler must return a correlated response.
    pub fn request(
        &self,
        to: &str,
        method: &str,
        payload: Value,
    ) -> Result<Value, IpcError> {
        let bus = self
            .bus
            .lock()
            .map_err(|_| IpcError::ServiceError("ipc bus lock poisoned".into()))?;
        let response = bus.send_request(&self.from_id, to, method, payload)?;
        if response.get("error").is_some() {
            // Structured service errors are still successful IPC round-trips.
            return Ok(response);
        }
        Ok(response)
    }

    /// Fire-and-forget event delivery to an endpoint.
    pub fn event(&self, to: &str, method: &str, payload: Value) {
        if let Ok(bus) = self.bus.lock() {
            bus.send_event(&self.from_id, to, method, payload);
        }
    }

    /// Validate that a payload is a response (not request/event). Used internally after send_request.
    #[allow(dead_code)]
    fn validate_response_kind(kind: IpcMessageKind) -> Result<(), IpcError> {
        if kind == IpcMessageKind::Response {
            Ok(())
        } else {
            Err(IpcError::ServiceError(format!(
                "expected response, got {:?}",
                kind
            )))
        }
    }
}
