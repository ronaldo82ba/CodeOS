use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct SettingsStore {
    values: HashMap<String, String>,
}

impl SettingsStore {
    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(String::as_str)
    }

    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.values.insert(key.into(), value.into());
    }
}
