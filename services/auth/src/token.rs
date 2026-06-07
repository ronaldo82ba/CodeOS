use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub name: String,
    pub scope: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    pub id: Uuid,
    pub app_id: String,
    pub capabilities: Vec<Capability>,
}

pub struct TokenIssuer;

impl TokenIssuer {
    pub fn issue(app_id: &str, capabilities: Vec<Capability>) -> AuthToken {
        AuthToken {
            id: Uuid::new_v4(),
            app_id: app_id.into(),
            capabilities,
        }
    }
}
