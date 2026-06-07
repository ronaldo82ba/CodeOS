use std::fmt::Display;

use serde_json::{json, Value};

use crate::ipc::IpcMessage;

/// Build a successful IPC response payload (fields returned directly).
pub fn ok(payload: Value) -> Value {
    payload
}

/// Build a structured IPC error payload.
pub fn err(code: &str, message: impl Display) -> Value {
    json!({
        "error": {
            "code": code,
            "message": message.to_string()
        }
    })
}

pub fn is_error(payload: &Value) -> bool {
    payload.get("error").and_then(|e| e.get("code")).is_some()
}

pub fn error_code(payload: &Value) -> Option<&str> {
    payload
        .get("error")
        .and_then(|e| e.get("code"))
        .and_then(|c| c.as_str())
}

pub fn error_message(payload: &Value) -> Option<&str> {
    payload
        .get("error")
        .and_then(|e| e.get("message"))
        .and_then(|m| m.as_str())
}

pub fn response(reply_to: &IpcMessage, payload: Value) -> IpcMessage {
    IpcMessage::new_response(reply_to, payload)
}

pub fn error_response(reply_to: &IpcMessage, code: &str, message: impl Display) -> IpcMessage {
    IpcMessage::new_response(reply_to, err(code, message))
}

pub fn require_str<'a>(payload: &'a Value, field: &str) -> Result<&'a str, String> {
    payload
        .get(field)
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("missing or invalid field: {field}"))
}

pub fn require_u64(payload: &Value, field: &str) -> Result<u64, String> {
    payload
        .get(field)
        .and_then(|v| v.as_u64())
        .ok_or_else(|| format!("missing or invalid field: {field}"))
}

pub fn require_bool(payload: &Value, field: &str) -> Result<bool, String> {
    payload
        .get(field)
        .and_then(|v| v.as_bool())
        .ok_or_else(|| format!("missing or invalid field: {field}"))
}
