#!/usr/bin/env bash
# Install stellar-cli v26.0.0 binary (matches CI). Idempotent.
set -euo pipefail

VERSION="${STELLAR_CLI_VERSION:-26.0.0}"
BIN_DIR="${HOME}/.local/bin"
mkdir -p "$BIN_DIR"

case "$(uname -s)-$(uname -m)" in
  Linux-x86_64)  ARCH=x86_64-unknown-linux-gnu ;;
  Linux-aarch64) ARCH=aarch64-unknown-linux-gnu ;;
  Darwin-x86_64) ARCH=x86_64-apple-darwin ;;
  Darwin-arm64)  ARCH=aarch64-apple-darwin ;;
  *)
    echo "Unsupported platform; try: cargo install stellar-cli --locked"
    exit 1
    ;;
esac

FILE="stellar-cli-${VERSION}-${ARCH}.tar.gz"
URL="https://github.com/stellar/stellar-cli/releases/download/v${VERSION}/${FILE}"

if command -v stellar >/dev/null 2>&1; then
  current="$(stellar --version 2>/dev/null | head -1 || true)"
  if echo "$current" | grep -q "$VERSION"; then
    echo "stellar-cli $VERSION already installed"
    exit 0
  fi
fi

echo "Downloading $URL"
curl -fL "$URL" | tar xz -C "$BIN_DIR"
export PATH="$BIN_DIR:$PATH"
stellar --version
echo "Add to PATH if needed: export PATH=\"$BIN_DIR:\$PATH\""
