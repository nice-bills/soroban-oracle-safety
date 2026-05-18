# Deep research — soroban-oracle-safety

Read before implementing. Sources verified May 2026.

## Problem statement

Blend’s Feb 2026 oracle manipulation exploit showed that **spot / thin liquidity oracles** on Soroban are unsafe for lending without **staleness checks, deviation limits, and TWAP-style averaging**. SDF’s Soroban MVE still lists oracles and lending primitives as core gaps.

This repo ships **reusable SEP-40 adapters** (not dashboards, not indexers).

## Standards & references

| Resource | URL | Use |
|----------|-----|-----|
| SEP-40 (Price Feed) | https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0040.md | `lastprice`, `price`, `prices`, `decimals`, `resolution` |
| sep-40-oracle crate | https://github.com/script3/sep-40-oracle | `PriceFeedTrait`, `MockPriceOracle` testutils |
| Blend oracle guide | https://docs-v1.blend.capital/pool-creators/selecting-an-oracle | Pools call `lastprice` with `Asset::Stellar(contract_id)` |
| Blend example aggregator | https://github.com/blend-capital/oracle-aggregator | `max_age`, `max_dev`, multi-source pattern |
| Stellar hello world | https://developers.stellar.org/docs/build/smart-contracts/getting-started/hello-world | Workspace layout, `stellar contract build` |
| Drips Stellar Wave | https://www.drips.network/wave/stellar | Apply repo **after** v0 ships; **do not** open Wave issues until accepted |

## Blend pool requirements (must satisfy)

From Blend docs:

1. Oracle must implement SEP-40 including **`lastprice`** and **`decimals`**.
2. Pool invokes oracle with **`Asset::Stellar(reserve_contract_address)`** for each reserve.
3. Pool oracle address is **immutable** after pool creation — adapters must be correct at deploy time.
4. Validate with `@blend-capital/blend-sdk` `PoolOracle.load(network, oracle_id, assets)`.

## Differentiation vs existing work

| Project | What it does | Our gap |
|---------|----------------|---------|
| blend-capital/oracle-aggregator | Reflector-like sources, `max_age`, `max_dev` | Not audited; example only; no TWAP wrapper crate |
| Reflector | Price feeds | Upstream source, not safety policy library |
| Per-app circuit breakers (e.g. StellarSwipe) | App-specific | Not reusable SEP-40 wrappers |

## Version pinning (critical)

Workspace `Cargo.toml` pins:

- `soroban-sdk = "22.0.7"` (aligns with blend-capital/oracle-aggregator)
- `sep-40-oracle = "1.2.2"` in manifest; Cargo may resolve **1.4.x** — if `cargo test` fails on version skew, **web search** `sep-40-oracle soroban-sdk compatibility` and bump **both** in `[workspace.dependencies]` together. Do not guess.

Rust **MSRV**: install `wasm32v1-none` target (`rustup target add wasm32v1-none`). Stellar docs require Rust ≥ 1.84 for recent CLI.

## Soroban pitfalls (learned in scaffold)

1. **`Symbol` strings**: only `[a-zA-Z0-9_]` — **no hyphens** (byte 45 panics). Use `v0_ph` not `v0-placeholder`.
2. **`#![no_std]`** on all contract crates; no `std::vec::Vec`.
3. **Contract function names** ≤ 32 chars.
4. **Floats** unsupported — use fixed-point `i128` with `decimals()`.
5. **SEP-40 errors**: return `Option::None` for bad input; consumers decide panic vs skip.
6. **Cross-contract**: use `PriceFeedClient` from `sep-40-oracle`; minimize host calls in hot paths.
7. **Test snapshots**: delete `contracts/*/test_snapshots/` when changing contract logic during dev.

## TWAP algorithm (AM-TWAP v0)

For asset `A` over `N` periods of length `resolution()` from source oracle:

1. `let records = source.prices(&A, N)?` (or walk `price` backward if sparse).
2. Reject if any `PriceData.timestamp` older than `max_staleness_secs` vs ledger timestamp.
3. AM-TWAP = arithmetic mean of `price` values (integer-safe: sum in i128, divide by count).
4. Return `PriceData { price, timestamp: ledger_time }` from `lastprice` interface.

GM-TWAP is **out of scope v0** (document in THREAT_MODEL as future).

## Circuit breaker rules (v0)

Wrapper stores config: `max_staleness_secs`, `max_deviation_bps`.

On `lastprice(asset)`:

1. `inner = source.lastprice(asset)?`
2. If `ledger_time - inner.timestamp > max_staleness_secs` → `None`
3. Load previous cached price for asset (optional storage); if deviation \> `max_deviation_bps` → `None`
4. Update cache; return `inner`

Admin: `set_config` (auth required). Constructor: `source_oracle`, config.

## Contract dependency graph

```
mock_feed (SEP-40 test double)
    ↑
circuit_breaker (wraps any SEP-40 address)
    ↑
twap_oracle (wraps SEP-40 source, implements PriceFeedTrait)
```

`twap_oracle` may optionally compose `circuit_breaker` on output in Phase 4 integration tests.

## Tooling

| Tool | Install |
|------|---------|
| stellar-cli | `cargo install stellar-cli --locked --features opt` or https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup |
| pnpm | corepack or https://pnpm.io — **only** under `packages/` and `examples/` |
| Python | **Not used in v0** — if added later: `uv venv` + `uv pip install` |

## Blocker protocol

On **first** build/test/deploy error you do not recognize:

1. Copy exact error text.
2. Web search: `stellar soroban` + error snippet.
3. Prefer official docs / GitHub issues / Stellar Discord threads.
4. Document fix in commit message or `docs/DEEP_RESEARCH.md` § Changelog.
5. **Never** guess version numbers or API shapes.

## Wave / issues policy

- **Do not** create GitHub issues for Drips until the repo is **accepted** on https://www.drips.network/wave/stellar.
- After acceptance: tag issues Trivial/Medium/High per https://docs.drips.network/wave/maintainers/participating-in-a-wave.
- See [WAVE.md](./WAVE.md).

## Changelog (scaffold)

- 2026-05-18: Pin `sep-40-oracle = "=1.2.2"` with `soroban-sdk 22.0.7` — 1.4.x pulls sdk 25 and breaks trait impls. `stellar-cli` install: drop `--features opt` (removed in v26).
- 2026-05-17: Initial research pack; Symbol hyphen pitfall; sep-40-oracle 1.4 resolves with sdk 22 in tests.
