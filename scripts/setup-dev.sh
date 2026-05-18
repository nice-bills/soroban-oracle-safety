#!/usr/bin/env bash
# Install toolchain for soroban-oracle-safety. Idempotent.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "==> Rust wasm target"
rustup target add wasm32v1-none 2>/dev/null || true

echo "==> Stellar CLI"
if ! command -v stellar >/dev/null 2>&1; then
  echo "Installing stellar-cli (cargo)..."
  cargo install stellar-cli --locked --features opt
fi
stellar --version

echo "==> pnpm (TypeScript)"
if ! command -v pnpm >/dev/null 2>&1; then
  if command -v corepack >/dev/null 2>&1; then
    corepack enable
    corepack prepare pnpm@9.15.0 --activate
  else
    echo "ERROR: install pnpm (https://pnpm.io/installation) or enable corepack"
    exit 1
  fi
fi
pnpm install

echo "==> Optional reference clones (.deps/, gitignored)"
mkdir -p .deps
for repo in blend-capital/oracle-aggregator script3/sep-40-oracle; do
  name="${repo##*/}"
  if [ ! -d ".deps/$name" ]; then
    git clone --depth 1 "https://github.com/$repo.git" ".deps/$name" || true
  fi
done

echo "Setup complete. Run: make verify"
