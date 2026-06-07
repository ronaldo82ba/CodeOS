use crate::client::IpcError;
use codecore::ipc::{error_code, is_error};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IpcClientError {
    #[error("ipc error: {0}")]
    Ipc(#[from] IpcError),
    #[error("ipc response error [{code}]: {message}")]
    Response { code: String, message: String },
    #[error("failed to decode response: {0}")]
    Decode(#[from] serde_json::Error),
}

impl IpcClientError {
    pub fn from_payload(payload: &serde_json::Value) -> Option<Self> {
        if is_error(payload) {
            Some(Self::Response {
                code: error_code(payload).unwrap_or("IPC_INTERNAL_ERROR").into(),
                message: codecore::ipc::error_message(payload)
                    .unwrap_or("unknown error")
                    .into(),
            })
        } else {
            None
        }
    }
}

pub fn check_response(payload: &serde_json::Value) -> Result<(), IpcClientError> {
    if let Some(err) = IpcClientError::from_payload(payload) {
        Err(err)
    } else {
        Ok(())
    }
}
