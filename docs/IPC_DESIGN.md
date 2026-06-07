# CodeOS IPC Design (v0.1)

CodeOS uses an **in-process message bus** for communication between apps, system UI, and system services. There is no shared memory in v0.1 — all payloads are JSON values carried by `IpcMessage`.

## Message model

```rust
pub struct IpcMessage {
    pub kind: IpcMessageKind,      // request | response | event
    pub id: String,
    pub correlation_id: Option<String>,
    pub from: String,
    pub to: String,
    pub method: String,
    pub payload: Value,
}
```

Constructors: `IpcMessage::new_request`, `new_response`, `new_event`.

| Kind | Direction | Response expected |
|------|-----------|-------------------|
| `request` | Client → service | Yes — handler returns `Some(response)` |
| `response` | Service → client | No |
| `event` | Service → subscribers | No |

## Endpoints

System services register on well-known endpoints (`codesvc.*`):

| Endpoint | Service |
|----------|---------|
| `codesvc.window` | Window / surface management |
| `codesvc.app` | App lifecycle |
| `codesvc.pkg` | Package install |
| `codesvc.notif` | Notifications |
| `codesvc.auth` | Permissions |
| `codesvc.storage` | App-scoped files |

Registration uses `IpcBus::register_endpoint(id, handler)`.

## Request / response flow (sync handler-return)

v0.1 uses **synchronous in-process handlers** — no pending-channel map or async wait.

1. Client calls `IpcBus::send_request(from, to, method, payload)` (via `IpcClient::request`).
2. Bus delivers the request to the registered handler for `to`.
3. Handler returns `Some(IpcMessage::new_response(&request, payload))`.
4. Bus returns the response payload directly to the caller.

If the handler returns `None` or a non-response kind, the bus returns `IpcBusError::NoResponse` or `InvalidResponse`. The typed client maps these to `IpcError::NoResponse` / `IpcError::ServiceError`.

Typed clients in `framework/ipc_client` wrap this with serde structs. Service traits return `Option<T>` — a `None` means the IPC round-trip failed or the response could not be decoded.

### Typed client usage

```rust
use codeos_ipc_client::{
    AppManagerClient, AppManagerService, IpcClient, StartAppRequest,
};

let client = IpcClient::new("app:com.example.demo");
let appmgr = AppManagerClient::new(client.clone());
let resp = appmgr.start_app(StartAppRequest {
    app_id: "com.example.other".into(),
});
if let Some(started) = resp {
    println!("instance: {}", started.instance_id);
}
```

Each service module (`window`, `app`, `pkg`, `notif`, `auth`, `storage`) exports a trait, request/response structs, and a `*Client` struct that owns an `IpcClient`. `WindowServiceClient::submit_frame` uses fire-and-forget `event()`; all other methods use synchronous `request()`.

### Handler signature

```rust
type IpcHandler = Box<dyn Fn(IpcMessage) -> Option<IpcMessage> + Send + Sync>;
```

Service handlers typically match on `msg.method` and return `Some(IpcMessage::new_response(...))` for requests, or `None` for events they do not reply to.

## Error responses

Errors use a structured payload (see `sdk/schemas/ipc/errors.json`):

```json
{
  "error": {
    "code": "IPC_INVALID_PAYLOAD",
    "message": "missing or invalid field: app_id"
  }
}
```

Shared codes: `IPC_NOT_FOUND`, `IPC_INVALID_PAYLOAD`, `IPC_PERMISSION_DENIED`, `IPC_INTERNAL_ERROR`, plus service-specific codes.

## Events (v0.1)

**Direct delivery:** `IpcBus::send_event(from, to, method, payload)` invokes the target endpoint handler; the return value is ignored.

**Broadcast:** Services emit domain events via `broadcast_event(from_endpoint, method, payload)`. Subscribers register by method name:

```rust
use codecore::ipc::subscribe_event;

let sub = subscribe_event("AppManager.AppStateChanged");
// ... trigger action ...
let event = sub.recv().unwrap();
```

The typed client exposes `EventSubscription` in `codeos_ipc_client::EventSubscription`.

### Event catalog

| Event | Source service |
|-------|----------------|
| `Window.SurfaceChanged` | codesvc.window |
| `AppManager.AppStateChanged` | codesvc.app |
| `Pkg.AppInstalled` / `Pkg.AppUninstalled` | codesvc.pkg |
| `Notif.NewNotification` / `Notif.NotificationCleared` | codesvc.notif |
| `Auth.PermissionChanged` | codesvc.auth |
| `Storage.FileChanged` | codesvc.storage |

## Global bus

```rust
use once_cell::sync::Lazy;

static GLOBAL_BUS: Lazy<Arc<Mutex<IpcBus>>> = ...;

pub fn get_global_bus() -> Arc<Mutex<IpcBus>>;
pub fn init_ipc_bus();
```

`codecore::init_core()` calls `init_ipc_bus()`. Each service binary calls `register_ipc_endpoint()` after `init_core()` to attach its handler.

Integration tests register the same handlers in-process without spawning separate processes.

## Schemas

Full method/event definitions: `sdk/schemas/ipc/*.json`.
