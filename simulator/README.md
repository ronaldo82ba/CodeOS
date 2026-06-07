# CodeSim — CodeOS Desktop Simulator

CodeSim v0.1 provides a Rust bootstrap that initializes CodeCore (IPC bus + service registry).

```bash
make sim
# or: cargo run -p codesim-desktop
```

## Layout

| Path | Purpose |
|------|---------|
| `desktop/` | Rust simulator bootstrap (`codesim` binary) |
| `assets/` | Static assets for future native UI |
| `legacy/` | Archived Electron + React prototype |

## v0.2

- Orchestrated spawn of all `codesvc.*` services
- Native UI bridge replacing Electron prototype
- Render pipeline to display `codeos-ui` scene graphs

See [docs/ARCHITECTURE.md](../docs/ARCHITECTURE.md).
