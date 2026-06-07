use codesvc_appmgr::{AppState, LifecycleManager};

#[test]
fn app_lifecycle_transitions() {
    assert_eq!(
        LifecycleManager::transition(AppState::Stopped, AppState::Starting).unwrap(),
        AppState::Starting
    );
    assert_eq!(
        LifecycleManager::transition(AppState::Starting, AppState::Running).unwrap(),
        AppState::Running
    );
}

#[test]
fn lifecycle_hooks_map_to_states() {
    assert_eq!(
        LifecycleManager::hook_to_state("on_start"),
        Some(AppState::Starting)
    );
    assert_eq!(
        LifecycleManager::hook_to_state("on_destroy"),
        Some(AppState::Destroyed)
    );
}
