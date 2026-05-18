# soroban-oracle-safety

Open-source **SEP-40 oracle safety adapters** for Stellar Soroban: staleness checks, deviation circuit breakers, and AM-TWAP — for Blend and other DeFi pools.

Not a dashboard. Not an indexer.

**Not audited.** Do not use in production without an independent security review.

## Components

| Contract | Role |
|----------|------|
| `mock_feed` | Admin-controlled SEP-40 test oracle |
| `twap_oracle` | AM-TWAP over source `prices()` history |
| `circuit_breaker` | Staleness + max-deviation wrapper |

Recommended chain for volatile reserves: **mock/reflector → twap_oracle → circuit_breaker**.

## Install

```bash
cd /home/bills/code/soroban-oracle-safety
make setup
```

Requires Rust ≥ 1.84, `wasm32v1-none` target, [Stellar CLI](https://developers.stellar.org/docs/tools/cli/install-cli), and pnpm.

## Build & test

```bash
make verify          # fmt, tests, wasm build, TypeScript
cargo test --workspace
stellar contract build
```

Per-crate tests:

```bash
cargo test -p mock-feed
cargo test -p circuit-breaker
cargo test -p twap-oracle
cargo test -p twap-oracle --test integration
```

## Testnet deploy (optional)

```bash
stellar keys generate default --network testnet   # once
stellar contract build
bash scripts/deploy-testnet.sh
```

Writes `configs/deployed.testnet.json` (gitignored). Initialize contracts via CLI invocations after deploy (admin, source, config).

## Blend oracle check (TypeScript)

```bash
export ORACLE_ID=C...           # deployed adapter (e.g. circuit_breaker)
export ASSETS=C...,C...         # reserve SAC contract ids
pnpm --filter @soroban-oracle-safety/ts run check-oracle
```

See [docs/BLEND_INTEGRATION.md](./docs/BLEND_INTEGRATION.md).

## Docs

- [AGENT.md](./AGENT.md) — build spec for agents
- [Architecture](./docs/ARCHITECTURE.md)
- [Threat model](./docs/THREAT_MODEL.md)
- [Deep research](./docs/DEEP_RESEARCH.md)

## License

MIT — see [LICENSE](./LICENSE).
