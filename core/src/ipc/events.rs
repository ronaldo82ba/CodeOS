use std::collections::HashMap;
use std::sync::{mpsc, Mutex, OnceLock};

use serde_json::Value;

use crate::ipc::IpcMessage;

type EventSubscribers = Mutex<HashMap<String, Vec<mpsc::Sender<IpcMessage>>>>;

static EVENT_SUBSCRIBERS: OnceLock<EventSubscribers> = OnceLock::new();

fn subscribers() -> &'static EventSubscribers {
    EVENT_SUBSCRIBERS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Register a subscriber for a named IPC event (v0.1 broadcast pattern).
pub fn subscribe_event(event_method: &str) -> mpsc::Receiver<IpcMessage> {
    let (tx, rx) = mpsc::channel();
    subscribers()
        .lock()
        .expect("event subscribers lock poisoned")
        .entry(event_method.to_string())
        .or_default()
        .push(tx);
    rx
}

/// Broadcast an event to all subscribers registered for `method`.
pub fn broadcast_event(from: &str, method: &str, payload: Value) {
    let msg = IpcMessage::new_event(from, "codesvc.eventbus", method, payload);
    let subs = subscribers()
        .lock()
        .expect("event subscribers lock poisoned");
    if let Some(list) = subs.get(method) {
        for tx in list {
            let _ = tx.send(msg.clone());
        }
    }
}

/// Clear all event subscribers (integration-test isolation).
pub fn reset_event_subscribers() {
    subscribers()
        .lock()
        .expect("event subscribers lock poisoned")
        .clear();
}
