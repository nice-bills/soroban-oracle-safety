#!/usr/bin/env bash
# Run the same checks as .github/workflows/ci.yml — use before push.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "=== ci-local: rust (fmt, test, clippy) ==="
rustup target add wasm32v1-none 2>/dev/null || true
rustup component add rustfmt clippy 2>/dev/null || true

cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings

echo "=== ci-local: stellar contract build ==="
if command -v stellar >/dev/null 2>&1; then
  stellar contract build
else
  echo "stellar CLI not found."
  echo "Install: curl -fL https://github.com/stellar/stellar-cli/releases/download/v26.0.0/stellar-cli-26.0.0-x86_64-unknown-linux-gnu.tar.gz | tar xz -C ~/.local/bin"
  echo "Or: cargo install stellar-cli --locked"
  echo "Fallback: cargo build --target wasm32v1-none --release -p mock-feed -p circuit-breaker -p twap-oracle"
  cargo build --target wasm32v1-none --release -p mock-feed -p circuit-breaker -p twap-oracle
fi

echo "=== ci-local: typescript ==="
if ! command -v pnpm >/dev/null 2>&1; then
  echo "ERROR: pnpm required (corepack enable && corepack prepare pnpm@9.15.0 --activate)"
  exit 1
fi
pnpm install --frozen-lockfile
pnpm --filter @soroban-oracle-safety/ts run build

echo "ci-local: OK"
