#!/usr/bin/env bash
# Build the full CodeOS Rust workspace
set -euo pipefail
cargo build --workspace "$@"
