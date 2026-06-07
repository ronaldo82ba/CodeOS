# CodeOS SDK — Developer Guide

## Typed IPC clients

Apps and services should use the official stubs in `codeos-ipc-client` instead of hand-crafted JSON.

```rust
use codeos_ipc_client::{
    AppManagerClient, AppManagerService, IpcClient, StartAppRequest,
    StorageService, StorageServiceClient, WriteFileRequest,
};

let client = IpcClient::new("app:com.example.demo");
let appmgr = AppManagerClient::new(client.clone());
let storage = StorageServiceClient::new(client);

if let Some(resp) = appmgr.start_app(StartAppRequest {
    app_id: "com.example.other".into(),
}) {
    println!("started {}", resp.instance_id);
}

storage.write_file(WriteFileRequest {
    app_id: "com.example.demo".into(),
    path: "state.json".into(),
    data: "e30=".into(),
});
```

### Service traits and clients

| Module | Trait | Client |
|--------|-------|--------|
| `window` | `WindowService` | `WindowServiceClient` |
| `app` | `AppManagerService` | `AppManagerClient` |
| `pkg` | `PackageService` | `PackageServiceClient` |
| `notif` | `NotificationService` | `NotificationServiceClient` |
| `auth` | `AuthService` | `AuthServiceClient` |
| `storage` | `StorageService` | `StorageServiceClient` |

`IpcClient` is `Clone` — create multiple service clients from one connection identity.

See [IPC_DESIGN.md](../../docs/IPC_DESIGN.md) for the bus model and event subscriptions.
