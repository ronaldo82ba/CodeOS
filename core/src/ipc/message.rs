use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IpcMessageKind {
    Request,
    Response,
    Event,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IpcMessage {
    pub kind: IpcMessageKind,
    pub id: String,
    pub correlation_id: Option<String>,
    pub from: String,
    pub to: String,
    pub method: String,
    pub payload: Value,
}

impl IpcMessage {
    pub fn new_request(
        from: impl Into<String>,
        to: impl Into<String>,
        method: impl Into<String>,
        payload: Value,
    ) -> Self {
        Self {
            kind: IpcMessageKind::Request,
            id: Uuid::new_v4().to_string(),
            correlation_id: None,
            from: from.into(),
            to: to.into(),
            method: method.into(),
            payload,
        }
    }

    pub fn new_response(request: &IpcMessage, payload: Value) -> Self {
        Self {
            kind: IpcMessageKind::Response,
            id: Uuid::new_v4().to_string(),
            correlation_id: Some(request.id.clone()),
            from: request.to.clone(),
            to: request.from.clone(),
            method: request.method.clone(),
            payload,
        }
    }

    pub fn new_event(
        from: impl Into<String>,
        to: impl Into<String>,
        method: impl Into<String>,
        payload: Value,
    ) -> Self {
        Self {
            kind: IpcMessageKind::Event,
            id: Uuid::new_v4().to_string(),
            correlation_id: None,
            from: from.into(),
            to: to.into(),
            method: method.into(),
            payload,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn response_carries_correlation_id() {
        let req = IpcMessage::new_request(
            "app:test",
            "codesvc.app",
            "AppManager.StartApp",
            json!({}),
        );
        let resp = IpcMessage::new_response(&req, json!({ "ok": true }));
        assert_eq!(resp.correlation_id.as_deref(), Some(req.id.as_str()));
        assert_eq!(resp.from, "codesvc.app");
        assert_eq!(resp.to, "app:test");
    }
}
