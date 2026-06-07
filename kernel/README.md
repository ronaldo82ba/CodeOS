# CodeOS Kernel Layer (CodeKernel)

Linux-based kernel configuration, patches, and device trees for CodeOS targets.

See [linux-notes.md](linux-notes.md) for boot chain and v0.2 plans.

## Layout

- **configs/** — Kernel defconfigs for each target
- **patches/** — CodeOS-specific kernel patches
- **dts/** — Device tree sources
- **linux-notes.md** — Boot sequence and integration notes

## Targets

| Config | Target |
|--------|--------|
| `codeos-simulator-defconfig` | Development / CI (minimal) |
| `codeos-arm64-defconfig` | QEMU `virt` and reference ARM64 boards |

## v0.1 Status

Configuration stubs only. Full kernel build pipeline and `codeos-init` land in v0.2.
