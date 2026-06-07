# CodeOS

[https://github.com/Ronaldo82ba/CodeOS](https://github.com/Ronaldo82ba/CodeOS)

**A true operating system with its own userspace, app model, and UI.**

CodeOS is not Android, not a ROM, and not a skin. It ships its own kernel boundary (CodeKernel), service-oriented core (CodeCore), modular system services (CodeServices), application framework (CodeFramework), system UI (CodeUI), developer SDK (CodeSDK), and desktop simulator (CodeSim).

> Author: Ronaldo Mijares · Version: **0.1.0**

---

## Vision

- **True OS** — owned userspace, app model, and UI stack
- **Simulator-first** — develop on desktop; port to Linux + QEMU ARM later
- **Service-oriented core** — CodeCore boot, IPC bus, service registry
- **Message-based IPC** — no shared memory in v0.1
- **First-class apps** — `.capp` packages with `codeos_manifest.toml`

---

## Repository Layout

```
codeos/
├── core/              CodeCore — boot, IPC, registry
├── services/          CodeServices — codesvc.*
├── framework/         CodeFramework — runtime, UI, IPC client
├── system_ui/         CodeUI — shell, settings
├── sdk/               CodeSDK — CLI, templates, docs
├── simulator/         CodeSim — desktop bootstrap + assets
├── kernel/            CodeKernel — Linux configs, notes
├── docs/              Architecture and specifications
└── tools/             Scripts, CI, integration tests
```

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for the full layer model.

---

## Quick Start

### Prerequisites

| Tool | Version | Purpose |
|------|---------|---------|
| Rust | ≥ 1.75 | Core, services, framework, CLI |
| Make | any | Convenience targets |

### Build

```bash
make build
# or: cargo build --workspace
```

### Run the simulator

```bash
make sim
# or: cargo run -p codesim-desktop
```

### Create and run an app

```bash
cargo run -p codeos-cli -- new myapp
cd myapp
cargo run -p codeos-cli -- build
cargo run -p codeos-cli -- run --simulator
```

### Tests

```bash
make test
```

---

## Documentation

- [ARCHITECTURE.md](docs/ARCHITECTURE.md)
- [APP_MODEL.md](docs/APP_MODEL.md)
- [IPC_DESIGN.md](docs/IPC_DESIGN.md)
- [SERVICES.md](docs/SERVICES.md)
- [UI_FRAMEWORK.md](docs/UI_FRAMEWORK.md)
- [SDK_OVERVIEW.md](docs/SDK_OVERVIEW.md)
- [ROADMAP.md](docs/ROADMAP.md)

---

## License

Apache-2.0 — see [LICENSE](LICENSE).
