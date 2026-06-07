use codecore::services::registry::{init_registry, register_service, lookup_service};
use codecore::services::types::{names, ServiceDescriptor, ServiceState};

#[test]
fn registers_codesvc_endpoints() {
    init_registry();
    register_service(ServiceDescriptor::new(names::WINDOW, 100)).unwrap();
    register_service(ServiceDescriptor::new(names::APP, 101)).unwrap();

    let window = lookup_service(names::WINDOW).unwrap();
    assert_eq!(window.pid, 100);
    assert_eq!(window.state, ServiceState::Starting);

    let app = lookup_service(names::APP).unwrap();
    assert_eq!(app.name, "codesvc.app");
}
