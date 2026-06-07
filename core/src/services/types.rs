use serde::{Deserialize, Serialize};

/// Well-known CodeServices endpoint names (codesvc.*).
pub mod names {
    pub const WINDOW: &str = "codesvc.window";
    pub const APP: &str = "codesvc.app";
    pub const PKG: &str = "codesvc.pkg";
    pub const NOTIF: &str = "codesvc.notif";
    pub const AUTH: &str = "codesvc.auth";
    pub const STORAGE: &str = "codesvc.storage";
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceState {
    Starting,
    Running,
    Stopping,
    Stopped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDescriptor {
    pub name: String,
    pub pid: u32,
    pub state: ServiceState,
    #[serde(default)]
    pub capabilities: Vec<String>,
}

impl ServiceDescriptor {
    pub fn new(name: impl Into<String>, pid: u32) -> Self {
        Self {
            name: name.into(),
            pid,
            state: ServiceState::Starting,
            capabilities: Vec::new(),
        }
    }
}
