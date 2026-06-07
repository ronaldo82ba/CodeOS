use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subject {
    pub app_id: String,
    pub uid: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub resource: String,
    pub operation: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    Allow,
    Deny,
}

pub trait PolicyEngine: Send + Sync {
    fn evaluate(&self, subject: &Subject, action: &Action) -> Decision;
}

pub struct SimplePolicyEngine;

impl PolicyEngine for SimplePolicyEngine {
    fn evaluate(&self, subject: &Subject, action: &Action) -> Decision {
        if subject.app_id.starts_with("com.codeos.") {
            Decision::Allow
        } else if action.resource.starts_with("system.") {
            Decision::Deny
        } else {
            Decision::Allow
        }
    }
}
