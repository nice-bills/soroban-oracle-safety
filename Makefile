.PHONY: help setup test build optimize verify fmt clippy clean

help:
	@echo "Targets: setup | test | build | optimize | verify | fmt | clippy | clean"

setup:
	bash scripts/setup-dev.sh

test:
	cargo test --workspace

build:
	stellar contract build

optimize:
	@for wasm in target/wasm32v1-none/release/*.wasm; do \
		[ -f "$$wasm" ] || continue; \
		stellar contract optimize --wasm "$$wasm"; \
	done

verify:
	bash scripts/verify-all.sh

fmt:
	cargo fmt --all

clippy:
	cargo clippy --workspace --all-targets -- -D warnings

clean:
	cargo clean
	rm -rf target/wasm32v1-none/release/*.optimized.wasm
