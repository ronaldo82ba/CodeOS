use crate::sandbox::{SandboxContext, SandboxError, SandboxLevel};
use crate::LoadedApp;

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("entry not found: {0}")]
    EntryNotFound(String),
    #[error("sandbox error: {0}")]
    Sandbox(#[from] SandboxError),
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub app_id: String,
    pub sandbox: SandboxLevel,
}

pub struct AppRuntime {
    app: LoadedApp,
    config: AppConfig,
}

impl AppRuntime {
    pub fn new(app: LoadedApp, config: AppConfig) -> Self {
        Self { app, config }
    }

    pub fn app_id(&self) -> &str {
        &self.config.app_id
    }

    pub fn prepare(&self) -> Result<SandboxContext, RuntimeError> {
        if !self.app.entry.exists() {
            return Err(RuntimeError::EntryNotFound(
                self.app.entry.display().to_string(),
            ));
        }
        Ok(SandboxContext::new(
            self.config.app_id.clone(),
            self.config.sandbox,
        )?)
    }

    pub fn launch_command(&self) -> Vec<String> {
        vec![
            self.app.entry.display().to_string(),
            "--codeos-app-id".into(),
            self.config.app_id.clone(),
        ]
    }
}
