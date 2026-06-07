use std::path::{Path, PathBuf};

use codesvc_pkg::CodeOsManifest;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("manifest not found")]
    ManifestNotFound,
    #[error("invalid manifest: {0}")]
    InvalidManifest(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Clone)]
pub struct LoadedApp {
    pub manifest: CodeOsManifest,
    pub root: PathBuf,
    pub entry: PathBuf,
}

pub struct AppLoader;

impl AppLoader {
    pub fn load(app_root: impl AsRef<Path>) -> Result<LoadedApp, LoadError> {
        let root = app_root.as_ref().to_path_buf();
        let manifest_path = root.join("codeos_manifest.toml");
        let data =
            std::fs::read_to_string(&manifest_path).map_err(|_| LoadError::ManifestNotFound)?;
        let manifest =
            CodeOsManifest::from_toml(&data).map_err(|e| LoadError::InvalidManifest(e.to_string()))?;
        manifest
            .validate()
            .map_err(LoadError::InvalidManifest)?;
        let entry = root.join(&manifest.entry.binary);
        Ok(LoadedApp {
            manifest,
            entry,
            root,
        })
    }

    pub fn load_capp(
        capp_path: impl AsRef<Path>,
        extract_to: impl AsRef<Path>,
    ) -> Result<LoadedApp, LoadError> {
        let extract_root = extract_to.as_ref();
        std::fs::create_dir_all(extract_root)?;
        // v0.1: extraction delegated to codesvc.pkg in production path
        let _ = capp_path.as_ref();
        Self::load(extract_root)
    }
}
