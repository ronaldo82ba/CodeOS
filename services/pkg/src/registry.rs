use std::collections::HashMap;

use std::path::{Path, PathBuf};

use std::sync::{Arc, Mutex};



use once_cell::sync::OnceCell;

use serde::{Deserialize, Serialize};



use crate::manifest::CodeOsManifest;



#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]

pub struct InstalledApp {

    pub app_id: String,

    pub name: String,

    pub version: String,

    pub permissions: HashMap<String, bool>,

    pub install_path: PathBuf,

    pub manifest: CodeOsManifest,

}



impl InstalledApp {

    pub fn manifest_path(&self) -> PathBuf {

        self.install_path.join("codeos_manifest.toml")

    }



    pub fn entry_binary_path(&self) -> PathBuf {

        self.install_path.join(&self.manifest.entry.binary)

    }



    pub fn granted_permission_keys(&self) -> Vec<String> {

        self.permissions

            .iter()

            .filter(|(_, granted)| **granted)

            .map(|(key, _)| key.clone())

            .collect()

    }

}



pub fn apps_dir() -> PathBuf {

    PathBuf::from(

        std::env::var("CODEOS_APPS_DIR").unwrap_or_else(|_| "./data/apps".into()),

    )

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



    pub fn register(&mut self, app: InstalledApp) -> Result<(), String> {

        if self.apps.contains_key(&app.app_id) {

            return Err(format!("app already installed: {}", app.app_id));

        }

        self.apps.insert(app.app_id.clone(), app);

        Ok(())

    }



    pub fn get(&self, app_id: &str) -> Option<&InstalledApp> {

        self.apps.get(app_id)

    }



    pub fn remove(&mut self, app_id: &str) -> Option<InstalledApp> {

        self.apps.remove(app_id)

    }



    pub fn contains(&self, app_id: &str) -> bool {
        self.apps.contains_key(app_id)
    }

    pub fn clear(&mut self) {
        self.apps.clear();
    }
}



static GLOBAL_REGISTRY: OnceCell<Arc<Mutex<AppRegistry>>> = OnceCell::new();



pub fn get_registry() -> Arc<Mutex<AppRegistry>> {

    GLOBAL_REGISTRY

        .get_or_init(|| Arc::new(Mutex::new(AppRegistry::new(apps_dir()))))

        .clone()

}



pub fn lookup_app(app_id: &str) -> Option<InstalledApp> {

    get_registry()

        .lock()

        .expect("pkg registry lock poisoned")

        .get(app_id)

        .cloned()

}



pub fn install_dir_for(app_id: &str) -> PathBuf {

    apps_dir().join(app_id)

}


