use std::collections::HashMap;



use serde::{Deserialize, Serialize};



#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]

pub struct AppSection {

    pub id: String,

    pub name: String,

    pub version: String,

    pub min_os_version: String,

}



#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]

pub struct EntrySection {

    pub binary: String,

    #[serde(default)]

    pub args: Vec<String>,

}



#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]

pub struct UiSection {

    pub main_view: String,

    pub icon: String,

}



#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]

pub struct PermissionsSection {

    #[serde(default)]

    pub network: bool,

    #[serde(default)]

    pub storage: bool,

    #[serde(default)]

    pub notifications: bool,

}



impl PermissionsSection {

    pub fn as_hashmap(&self) -> HashMap<String, bool> {

        HashMap::from([

            ("network".into(), self.network),

            ("storage".into(), self.storage),

            ("notifications".into(), self.notifications),

        ])

    }



    pub fn granted_keys(&self) -> Vec<String> {

        self.as_hashmap()

            .into_iter()

            .filter(|(_, granted)| *granted)

            .map(|(key, _)| key)

            .collect()

    }

}



#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]

pub struct MetadataSection {

    pub author: String,

    pub website: String,

    pub description: String,

}



#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]

pub struct CodeOsManifest {

    pub app: AppSection,

    pub entry: EntrySection,

    #[serde(default)]

    pub ui: Option<UiSection>,

    #[serde(default)]

    pub permissions: PermissionsSection,

    #[serde(default)]

    pub metadata: Option<MetadataSection>,

}



impl CodeOsManifest {

    pub fn from_toml(content: &str) -> Result<Self, toml::de::Error> {

        toml::from_str(content)

    }



    pub fn validate(&self) -> Result<(), String> {

        if self.app.id.is_empty() {

            return Err("app.id is required".into());

        }

        if self.app.name.is_empty() {

            return Err("app.name is required".into());

        }

        if self.app.version.is_empty() {

            return Err("app.version is required".into());

        }

        if self.entry.binary.is_empty() {

            return Err("entry.binary is required".into());

        }

        if !self.entry.binary.starts_with("bin/") {

            return Err("entry.binary must be under bin/".into());

        }

        Ok(())

    }

}



#[cfg(test)]

mod tests {

    use super::*;



    const SAMPLE: &str = "\
[app]\n\
id = \"com.ronaldo.myapp\"\n\
name = \"My App\"\n\
version = \"1.0.0\"\n\
min_os_version = \"0.1.0\"\n\
\n\
[entry]\n\
binary = \"bin/myapp\"\n\
args = []\n\
\n\
[ui]\n\
main_view = \"res/main_layout.json\"\n\
icon = \"assets/icon.png\"\n\
\n\
[permissions]\n\
network = true\n\
storage = true\n\
notifications = true\n\
\n\
[metadata]\n\
author = \"Ronaldo Mijares\"\n\
website = \"https://example.com\"\n\
description = \"Short description of the app.\"\n\
";



    #[test]

    fn parses_authoritative_manifest() {

        let manifest = CodeOsManifest::from_toml(SAMPLE).unwrap();

        assert_eq!(manifest.app.id, "com.ronaldo.myapp");

        assert_eq!(manifest.entry.binary, "bin/myapp");

        assert!(manifest.permissions.network);

        manifest.validate().unwrap();

    }

}


