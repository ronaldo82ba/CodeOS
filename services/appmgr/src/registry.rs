use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::lifecycle::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledApp {
    pub app_id: String,
    pub name: String,
    pub version: String,
    pub install_path: PathBuf,
    pub state: AppState,
}

pub struct AppRegistry {
    root: PathBuf,
    apps: HashMap<String, InstalledApp>,
}

impl AppRegistry {
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
            apps: HashMap::new(),
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn register(&mut self, app: InstalledApp) {
        self.apps.insert(app.app_id.clone(), app);
    }

    pub fn get(&self, app_id: &str) -> Option<&InstalledApp> {
        self.apps.get(app_id)
    }

    pub fn list(&self) -> Vec<&InstalledApp> {
        self.apps.values().collect()
    }

    pub fn set_state(&mut self, app_id: &str, state: AppState) -> Result<(), String> {
        let app = self
            .apps
            .get_mut(app_id)
            .ok_or_else(|| format!("app not found: {app_id}"))?;
        app.state = state;
        Ok(())
    }
}
