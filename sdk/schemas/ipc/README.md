# CodeOS IPC Schemas v0.1

Machine-readable schemas for all six CodeOS system services.

## Layout

| File | Service endpoint |
|------|------------------|
| `window.json` | `codesvc.window` |
| `app.json` | `codesvc.app` |
| `pkg.json` | `codesvc.pkg` |
| `notif.json` | `codesvc.notif` |
| `auth.json` | `codesvc.auth` |
| `storage.json` | `codesvc.storage` |
| `errors.json` | Shared error codes |

## Schema entry shape

Each method or event documents:

```json
{
  "method": "Service.Method",
  "payload": { },
  "response": { },
  "errors": [ "IPC_INVALID_PAYLOAD" ]
}
```

Events omit `response` and `errors`.

## Rust bindings

Typed clients live in `framework/ipc_client/`:

- Traits: `WindowService`, `AppManagerService`, `PackageService`, `NotificationService`, `AuthService`, `StorageService`
- Clients: `WindowServiceClient`, `AppManagerClient`, `PackageServiceClient`, `NotificationServiceClient`, `AuthServiceClient`, `StorageServiceClient`
- Method constants in `codeos_ipc_client::stubs`
- Event subscription via `EventSubscription::subscribe("AppManager.AppStateChanged")`

## Error responses

Failed requests return:

```json
{
  "error": {
    "code": "IPC_INVALID_PAYLOAD",
    "message": "missing or invalid field: app_id"
  }
}
```

See `errors.json` for the full code list.
