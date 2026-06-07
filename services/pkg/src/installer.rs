use std::fs::{self, File};

use std::io;

use std::path::Path;



use thiserror::Error;

use zip::ZipArchive;



use crate::registry::{get_registry, install_dir_for, InstalledApp};

use crate::validator::{CappValidator, ValidationError};



#[derive(Debug, Error)]

pub enum InstallError {

    #[error("validation failed: {0}")]

    Validation(#[from] ValidationError),

    #[error("app already installed: {0}")]

    AlreadyInstalled(String),

    #[error("io error: {0}")]

    Io(#[from] io::Error),

    #[error("zip error: {0}")]

    Zip(#[from] zip::result::ZipError),

}



pub struct InstallResult {

    pub app_id: String,

    pub version: String,

    pub installed: InstalledApp,

}



pub struct CappInstaller;



impl CappInstaller {

    pub fn install(capp_path: &Path) -> Result<InstallResult, InstallError> {

        let manifest = CappValidator::validate(capp_path)?;

        let app_id = manifest.app.id.clone();

        let version = manifest.app.version.clone();



        {

            let registry = get_registry();

            let reg = registry.lock().expect("pkg registry lock poisoned");

            if reg.contains(&app_id) {

                return Err(InstallError::AlreadyInstalled(app_id));

            }

        }



        let install_path = install_dir_for(&app_id);

        if install_path.exists() {

            fs::remove_dir_all(&install_path)?;

        }

        fs::create_dir_all(&install_path)?;



        Self::extract(capp_path, &install_path)?;



        let permissions = manifest.permissions.as_hashmap();

        codesvc_auth::grant_install_permissions(&app_id, &permissions);



        let installed = InstalledApp {

            app_id: app_id.clone(),

            name: manifest.app.name.clone(),

            version: version.clone(),

            permissions: permissions.clone(),

            install_path: install_path.clone(),

            manifest: manifest.clone(),

        };



        {

            let registry = get_registry();

            let mut reg = registry.lock().expect("pkg registry lock poisoned");

            reg.register(installed.clone())

                .map_err(|e| InstallError::AlreadyInstalled(e))?;

        }



        Ok(InstallResult {

            app_id,

            version,

            installed,

        })

    }



    fn extract(capp_path: &Path, dest: &Path) -> Result<(), InstallError> {

        let file = File::open(capp_path)?;

        let mut archive = ZipArchive::new(file)?;



        for i in 0..archive.len() {

            let mut entry = archive.by_index(i)?;

            let outpath = match entry.enclosed_name() {

                Some(path) => dest.join(path),

                None => continue,

            };



            if entry.is_dir() {

                fs::create_dir_all(&outpath)?;

            } else {

                if let Some(parent) = outpath.parent() {

                    fs::create_dir_all(parent)?;

                }

                let mut outfile = File::create(&outpath)?;

                io::copy(&mut entry, &mut outfile)?;

            }

        }



        Ok(())

    }

}


