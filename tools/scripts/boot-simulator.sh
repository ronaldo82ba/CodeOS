#!/usr/bin/env bash
# Boot CodeOS Simulator with system services
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"

echo "==> Building CodeOS services..."
cargo build --workspace --quiet

echo "==> Starting Simulator..."
cd "$ROOT/simulator"
pnpm install --silent 2>/dev/null || npm install
pnpm dev || npm run dev
