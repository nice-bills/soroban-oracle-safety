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

# Extract contract ID from stellar deploy output (JSON or plain text).
parse_contract_id() {
  local raw="$1"
  if command -v jq >/dev/null 2>&1; then
    local from_json
    from_json=$(printf '%s\n' "$raw" | jq -r '
      .contractId // .contract_id // .id //
      (.transactionHash // empty | select(length > 0)) // empty
    ' 2>/dev/null | head -1)
    if [[ -n "$from_json" && "$from_json" != "null" ]]; then
      printf '%s' "$from_json"
      return 0
    fi
  fi
  printf '%s\n' "$raw" | grep -oE 'C[A-Z0-9]{55}' | tail -1
}

deploy_wasm() {
  local wasm="$1"
  local out
  out=$(stellar contract deploy \
    --wasm "$wasm" \
    --source-account "$IDENTITY" \
    --network "$NETWORK" 2>&1) || {
    echo "$out" >&2
    return 1
  }
  local id
  id=$(parse_contract_id "$out")
  if [[ -z "$id" ]]; then
    echo "ERROR: could not parse contract ID from deploy output:" >&2
    echo "$out" >&2
    return 1
  fi
  echo "$id"
}

echo "==> Deploying mock_feed..."
MOCK_ID=$(deploy_wasm target/wasm32v1-none/release/mock_feed.wasm)
echo "    mock_feed: $MOCK_ID"

echo "==> Deploying twap_oracle (source=$MOCK_ID)..."
TWAP_ID=$(deploy_wasm target/wasm32v1-none/release/twap_oracle.wasm)
echo "    twap_oracle: $TWAP_ID"

echo "==> Deploying circuit_breaker (source=$TWAP_ID)..."
BREAKER_ID=$(deploy_wasm target/wasm32v1-none/release/circuit_breaker.wasm)
echo "    circuit_breaker: $BREAKER_ID"

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
