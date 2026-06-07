# CodeOS System Services API (v0.1)

IPC schemas for all six system services. Method names must match exactly.

Schemas: `sdk/schemas/ipc/`

## codesvc.window

| Kind | Method | Payload | Response |
|------|--------|---------|----------|
| Method | `Window.CreateSurface` | `{ app_id, width, height }` | `{ surface_id }` |
| Method | `Window.DestroySurface` | `{ surface_id }` | `{}` |
| Method | `Window.SubmitFrame` | `{ surface_id, frame_data }` | `{}` |
| Event | `Window.SurfaceChanged` | `{ surface_id, width, height }` | — |

`Window.SubmitFrame` is accepted as an **Event** (fire-and-forget) or Request; v0.1 delivers frames to a pluggable `FrameSink` (default: log-only). CodeSim registers `CallbackFrameSink` → `SimulatorRenderer::present_frame` at boot.

## codesvc.app

| Kind | Method | Payload | Response |
|------|--------|---------|----------|
| Method | `AppManager.StartApp` | `{ app_id }` | `{ instance_id }` |
| Method | `AppManager.StopApp` | `{ instance_id }` | `{}` |
| Method | `AppManager.ListRunningApps` | `{}` | `{ running: [{ app_id, instance_id, state }] }` |
| Event | `AppManager.AppStateChanged` | `{ instance_id, state }` | — |

`state` values: `foreground`, `background`, `paused`, `stopped`.

## codesvc.pkg

| Kind | Method | Payload | Response |
|------|--------|---------|----------|
| Method | `Pkg.Install` | `{ capp_path }` | `{ app_id, version }` |
| Method | `Pkg.Uninstall` | `{ app_id }` | `{}` |
| Method | `Pkg.GetAppInfo` | `{ app_id }` | `{ app_id, name, version, permissions }` |
| Event | `Pkg.AppInstalled` | `{ app_id }` | — |
| Event | `Pkg.AppUninstalled` | `{ app_id }` | — |

Install validates `.capp` as a ZIP archive with `codeos_manifest.toml` and the declared `entry.binary`, extracts to `CODEOS_APPS_DIR` (default `./data/apps/<app_id>/`), registers the app, and grants manifest permissions to `codesvc.auth`.

## codesvc.notif

| Kind | Method | Payload | Response |
|------|--------|---------|----------|
| Method | `Notif.Post` | `{ app_id, title, body, timestamp }` | `{ notif_id }` |
| Method | `Notif.Clear` | `{ notif_id }` | `{}` |
| Method | `Notif.List` | `{}` | `{ notifications: [...] }` |
| Event | `Notif.NewNotification` | `{ notif_id }` | — |
| Event | `Notif.NotificationCleared` | `{ notif_id }` | — |

## codesvc.auth

| Kind | Method | Payload | Response |
|------|--------|---------|----------|
| Method | `Auth.RequestPermission` | `{ app_id, permission }` | `{ granted }` |
| Method | `Auth.CheckPermission` | `{ app_id, permission }` | `{ granted }` |
| Event | `Auth.PermissionChanged` | `{ app_id, permission, granted }` | — |

## codesvc.storage

| Kind | Method | Payload | Response |
|------|--------|---------|----------|
| Method | `Storage.WriteFile` | `{ app_id, path, data }` | `{}` |
| Method | `Storage.ReadFile` | `{ app_id, path }` | `{ data }` |
| Method | `Storage.ListFiles` | `{ app_id, path }` | `{ files }` |
| Event | `Storage.FileChanged` | `{ app_id, path }` | — |

`data` fields are base64-encoded.

## Typed Rust clients

```rust
use codeos_ipc_client::{AppManagerClient, EventSubscription, IpcClient};
use codeos_ipc_client::stubs::app::events;

let client = IpcClient::new("app:demo");
let apps = AppManagerClient::new(&client);
let sub = EventSubscription::subscribe(events::APP_STATE_CHANGED);

let resp = apps.start_app("com.example.app")?;
let event = sub.recv_timeout(Duration::from_secs(1))?;
```

See `docs/IPC_DESIGN.md` for bus architecture and error format.
