use std::path::PathBuf;

use super::level::SandboxLevel;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SandboxError {
    #[error("invalid sandbox configuration")]
    InvalidConfig,
}

#[derive(Debug, Clone)]
pub struct SandboxContext {
    pub app_id: String,
    pub level: SandboxLevel,
    pub data_dir: PathBuf,
    pub uid: u32,
}

impl SandboxContext {
    pub fn new(app_id: String, level: SandboxLevel) -> Result<Self, SandboxError> {
        let home = std::env::var("USERPROFILE")
            .or_else(|_| std::env::var("HOME"))
            .unwrap_or_else(|_| ".".into());
        let data_dir = PathBuf::from(format!("{home}/.codeos/data/{app_id}"));
        Ok(Self {
            uid: hash_app_id(&app_id),
            app_id,
            level,
            data_dir,
        })
    }

    pub fn ensure_data_dir(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.data_dir)
    }
}

fn hash_app_id(app_id: &str) -> u32 {
    app_id.bytes().fold(10_000u32, |acc, b| {
        acc.wrapping_mul(31).wrapping_add(b as u32)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_sandbox_context() {
        let ctx = SandboxContext::new("com.example.app".into(), SandboxLevel::Standard).unwrap();
        assert_eq!(ctx.app_id, "com.example.app");
        assert!(ctx.uid >= 10_000);
    }
}
