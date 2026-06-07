use std::fs::File;
use std::io::Read;
use std::path::Path;

use thiserror::Error;
use zip::ZipArchive;

use crate::manifest::CodeOsManifest;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("not a valid zip archive: {0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("missing required file: codeos_manifest.toml")]
    MissingManifest,
    #[error("manifest parse error: {0}")]
    ManifestParse(String),
    #[error("manifest validation error: {0}")]
    ManifestInvalid(String),
    #[error("missing required binary: {0}")]
    MissingBinary(String),
}

pub struct CappValidator;

impl CappValidator {
    pub fn validate(capp_path: &Path) -> Result<CodeOsManifest, ValidationError> {
        let file = File::open(capp_path)?;
        let mut archive = ZipArchive::new(file)?;

        let manifest = Self::read_manifest(&mut archive)?;
        manifest
            .validate()
            .map_err(ValidationError::ManifestInvalid)?;

        if archive.by_name(&manifest.entry.binary).is_err() {
            return Err(ValidationError::MissingBinary(
                manifest.entry.binary.clone(),
            ));
        }

        Ok(manifest)
    }

    fn read_manifest(archive: &mut ZipArchive<File>) -> Result<CodeOsManifest, ValidationError> {
        let mut manifest_file = archive
            .by_name("codeos_manifest.toml")
            .map_err(|_| ValidationError::MissingManifest)?;
        let mut content = String::new();
        manifest_file.read_to_string(&mut content)?;
        CodeOsManifest::from_toml(&content).map_err(|e| ValidationError::ManifestParse(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;
    use crate::manifest::{AppSection, CodeOsManifest, EntrySection};
    use zip::write::SimpleFileOptions;
    use zip::ZipWriter;

    #[test]
    fn rejects_missing_binary() {
        let manifest = CodeOsManifest {
            app: AppSection {
                id: "com.test.app".into(),
                name: "Test".into(),
                version: "1.0.0".into(),
                min_os_version: "0.1.0".into(),
            },
            entry: EntrySection {
                binary: "bin/missing".into(),
                args: vec![],
            },
            ui: None,
            permissions: Default::default(),
            metadata: None,
        };

        let dir = std::env::temp_dir().join(format!("codeos-validator-{}", uuid_simple()));
        std::fs::create_dir_all(&dir).unwrap();
        let capp = dir.join("bad.capp");

        let file = std::fs::File::create(&capp).unwrap();
        let mut zip = ZipWriter::new(file);
        let options =
            SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
        let manifest_toml = toml::to_string_pretty(&manifest).unwrap();
        zip.start_file("codeos_manifest.toml", options).unwrap();
        zip.write_all(manifest_toml.as_bytes()).unwrap();
        zip.finish().unwrap();

        let err = CappValidator::validate(&capp).unwrap_err();
        assert!(matches!(err, ValidationError::MissingBinary(_)));

        let _ = std::fs::remove_dir_all(dir);
    }

    fn uuid_simple() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
    }
}
