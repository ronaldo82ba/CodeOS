use std::collections::HashMap;
use std::sync::RwLock;

use crate::services::types::{ServiceDescriptor, ServiceState};

#[derive(Debug, thiserror::Error)]
pub enum ServiceRegistryError {
    #[error("service already registered: {0}")]
    AlreadyRegistered(String),
    #[error("service not found: {0}")]
    NotFound(String),
}

struct RegistryInner {
    services: HashMap<String, ServiceDescriptor>,
    endpoints: HashMap<String, ()>,
}

static REGISTRY: std::sync::OnceLock<RwLock<RegistryInner>> = std::sync::OnceLock::new();

fn registry() -> &'static RwLock<RegistryInner> {
    REGISTRY.get_or_init(|| {
        RwLock::new(RegistryInner {
            services: HashMap::new(),
            endpoints: HashMap::new(),
        })
    })
}

pub fn init_registry() {
    let _ = registry();
    tracing::info!("service registry initialized");
}

pub fn register_service(descriptor: ServiceDescriptor) -> Result<(), ServiceRegistryError> {
    let mut reg = registry().write().expect("registry lock poisoned");
    if reg.services.contains_key(&descriptor.name) {
        return Err(ServiceRegistryError::AlreadyRegistered(descriptor.name));
    }
    tracing::info!(service = %descriptor.name, pid = descriptor.pid, "service registered");
    reg.services.insert(descriptor.name.clone(), descriptor);
    Ok(())
}

pub fn register_endpoint(endpoint: &str) {
    let mut reg = registry().write().expect("registry lock poisoned");
    reg.endpoints.insert(endpoint.to_string(), ());
}

pub fn lookup_service(name: &str) -> Option<ServiceDescriptor> {
    registry()
        .read()
        .expect("registry lock poisoned")
        .services
        .get(name)
        .cloned()
}

pub fn set_service_state(name: &str, state: ServiceState) -> Result<(), ServiceRegistryError> {
    let mut reg = registry().write().expect("registry lock poisoned");
    let entry = reg
        .services
        .get_mut(name)
        .ok_or_else(|| ServiceRegistryError::NotFound(name.into()))?;
    entry.state = state;
    Ok(())
}

pub fn list_services() -> Vec<ServiceDescriptor> {
    registry()
        .read()
        .expect("registry lock poisoned")
        .services
        .values()
        .cloned()
        .collect()
}
