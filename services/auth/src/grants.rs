use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

use once_cell::sync::Lazy;

static GRANTS: Lazy<Mutex<HashSet<(String, String)>>> = Lazy::new(|| Mutex::new(HashSet::new()));

pub fn grant_install_permissions(app_id: &str, permissions: &HashMap<String, bool>) {
    let mut grants = GRANTS.lock().expect("auth grants lock poisoned");
    for (permission, granted) in permissions {
        if *granted {
            grants.insert((app_id.to_string(), permission.clone()));
        }
    }
}

pub fn revoke_install_permissions(app_id: &str) {
    GRANTS
        .lock()
        .expect("auth grants lock poisoned")
        .retain(|(id, _)| id != app_id);
}

pub fn is_granted(app_id: &str, permission: &str) -> bool {
    GRANTS
        .lock()
        .expect("auth grants lock poisoned")
        .contains(&(app_id.to_string(), permission.to_string()))
}

pub fn grant(app_id: &str, permission: &str) {
    GRANTS
        .lock()
        .expect("auth grants lock poisoned")
        .insert((app_id.to_string(), permission.to_string()));
}
