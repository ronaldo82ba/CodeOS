use std::fs::File;

use std::io::{Read, Write};

use std::path::Path;



use zip::write::SimpleFileOptions;

use zip::ZipWriter;



use crate::manifest::CodeOsManifest;



#[derive(Debug, thiserror::Error)]

pub enum PackError {

    #[error("io error: {0}")]

    Io(#[from] std::io::Error),

    #[error("zip error: {0}")]

    Zip(#[from] zip::result::ZipError),

    #[error("manifest error: {0}")]

    Manifest(String),

}



pub struct CappPackager;



impl CappPackager {

    pub fn pack(

        manifest: &CodeOsManifest,

        output: &Path,

        payload_files: &[(&str, &[u8])],

    ) -> Result<(), PackError> {

        manifest.validate().map_err(PackError::Manifest)?;



        for (name, _) in payload_files {

            if *name == "codeos_manifest.toml" {

                return Err(PackError::Manifest(

                    "payload must not include codeos_manifest.toml".into(),

                ));

            }

        }



        if !payload_files

            .iter()

            .any(|(name, _)| *name == manifest.entry.binary)

        {

            return Err(PackError::Manifest(format!(

                "payload missing entry.binary: {}",

                manifest.entry.binary

            )));

        }



        let file = File::create(output)?;

        let mut zip = ZipWriter::new(file);

        let options =

            SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);



        let manifest_toml =

            toml::to_string_pretty(manifest).map_err(|e| PackError::Manifest(e.to_string()))?;

        zip.start_file("codeos_manifest.toml", options)?;

        zip.write_all(manifest_toml.as_bytes())?;



        for (name, data) in payload_files {

            zip.start_file(*name, options)?;

            zip.write_all(data)?;

        }



        zip.finish()?;

        Ok(())

    }



    pub fn read_manifest(capp_path: &Path) -> Result<CodeOsManifest, PackError> {

        let file = File::open(capp_path)?;

        let mut archive = zip::ZipArchive::new(file)?;

        let mut manifest_file = archive.by_name("codeos_manifest.toml")?;

        let mut content = String::new();

        manifest_file.read_to_string(&mut content)?;

        CodeOsManifest::from_toml(&content).map_err(|e| PackError::Manifest(e.to_string()))

    }

}


