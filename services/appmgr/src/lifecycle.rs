use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppState {
    Created,
    Stopped,
    Starting,
    Running,
    Paused,
    Stopping,
    Destroyed,
}

impl AppState {
    pub fn can_transition_to(&self, next: AppState) -> bool {
        use AppState::*;
        matches!(
            (self, next),
            (Created, Stopped)
                | (Stopped, Starting)
                | (Starting, Running)
                | (Starting, Stopped)
                | (Running, Paused)
                | (Running, Stopping)
                | (Paused, Running)
                | (Paused, Stopping)
                | (Stopping, Stopped)
                | (Stopped, Destroyed)
        )
    }
}

pub struct LifecycleManager;

impl LifecycleManager {
    pub fn transition(current: AppState, next: AppState) -> Result<AppState, String> {
        if current.can_transition_to(next) {
            Ok(next)
        } else {
            Err(format!("invalid transition: {:?} -> {:?}", current, next))
        }
    }

    /// Map lifecycle hook names from codeos_manifest.toml to state transitions.
    pub fn hook_to_state(hook: &str) -> Option<AppState> {
        match hook {
            "on_create" => Some(AppState::Created),
            "on_start" => Some(AppState::Starting),
            "on_resume" => Some(AppState::Running),
            "on_pause" => Some(AppState::Paused),
            "on_stop" => Some(AppState::Stopping),
            "on_destroy" => Some(AppState::Destroyed),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_launch_transition() {
        assert_eq!(
            LifecycleManager::transition(AppState::Stopped, AppState::Starting).unwrap(),
            AppState::Starting
        );
    }

    #[test]
    fn rejects_invalid_transition() {
        assert!(LifecycleManager::transition(AppState::Stopped, AppState::Running).is_err());
    }
}
