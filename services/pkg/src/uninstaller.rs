use std::fs;
use std::path::PathBuf;

use thiserror::Error;

use crate::registry::{get_registry, InstalledApp};

#[derive(Debug, Error)]
pub enum UninstallError {
    #[error("app not found: {0}")]
    NotFound(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

pub struct CappUninstaller;

impl CappUninstaller {
    pub fn uninstall(app_id: &str) -> Result<InstalledApp, UninstallError> {
        let removed = {
            let registry = get_registry();
            let mut reg = registry.lock().expect("pkg registry lock poisoned");
            reg.remove(app_id)
                .ok_or_else(|| UninstallError::NotFound(app_id.to_string()))?
        };

        codesvc_auth::revoke_install_permissions(app_id);

        if removed.install_path.exists() {
            fs::remove_dir_all(&removed.install_path)?;
        }

        Ok(removed)
    }

    pub fn uninstall_path(app_id: &str) -> Result<PathBuf, UninstallError> {
        let removed = Self::uninstall(app_id)?;
        Ok(removed.install_path)
    }
}
