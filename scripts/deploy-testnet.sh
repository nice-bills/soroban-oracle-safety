#!/usr/bin/env bash
# Deploy mock_feed → twap_oracle → circuit_breaker on testnet (optional Phase 6).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

if ! command -v stellar >/dev/null 2>&1; then
  echo "ERROR: stellar CLI required (run make setup)"
  exit 1
fi

stellar contract build

CONFIG_DIR="$ROOT/configs"
mkdir -p "$CONFIG_DIR"
OUT="$CONFIG_DIR/deployed.testnet.json"

IDENTITY="${STELLAR_IDENTITY:-default}"
NETWORK="${STELLAR_NETWORK:-testnet}"

echo "==> Deploying mock_feed..."
MOCK_ID=$(stellar contract deploy \
  --wasm target/wasm32v1-none/release/mock_feed.wasm \
  --source-account "$IDENTITY" \
  --network "$NETWORK" | tail -1)

echo "==> Deploying twap_oracle (source=$MOCK_ID)..."
TWAP_ID=$(stellar contract deploy \
  --wasm target/wasm32v1-none/release/twap_oracle.wasm \
  --source-account "$IDENTITY" \
  --network "$NETWORK" | tail -1)

echo "==> Deploying circuit_breaker (source=$TWAP_ID)..."
BREAKER_ID=$(stellar contract deploy \
  --wasm target/wasm32v1-none/release/circuit_breaker.wasm \
  --source-account "$IDENTITY" \
  --network "$NETWORK" | tail -1)

cat >"$OUT" <<EOF
{
  "network": "$NETWORK",
  "mock_feed": "$MOCK_ID",
  "twap_oracle": "$TWAP_ID",
  "circuit_breaker": "$BREAKER_ID"
}
EOF

echo "Wrote $OUT"
cat "$OUT"
