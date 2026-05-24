#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "==> fmt check"
cargo fmt --all -- --check

echo "==> workspace tests"
cargo test --workspace --features testutils

echo "==> clippy"
cargo clippy --workspace --all-targets -- -D warnings

echo "==> contract build"
if command -v stellar >/dev/null 2>&1; then
  stellar contract build
else
  echo "WARN: stellar CLI missing — cargo wasm build fallback"
  cargo build --target wasm32v1-none --release -p mock-feed -p circuit-breaker -p twap-oracle
fi

echo "==> TypeScript"
if [ -f pnpm-lock.yaml ]; then
  pnpm -r run build 2>/dev/null || pnpm --filter @soroban-oracle-safety/ts run build
fi

echo "verify-all: OK"
