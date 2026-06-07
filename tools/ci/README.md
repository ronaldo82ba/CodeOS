# CI configuration stubs for CodeOS v0.1

Planned pipeline stages:

1. `cargo build --workspace`
2. `cargo test --workspace`
3. `cargo clippy --workspace -- -D warnings` (v0.2)
4. Simulator smoke test via `make sim`

GitHub Actions workflow to be added in v0.2.
