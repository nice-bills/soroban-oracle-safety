# Build spec — soroban-oracle-safety

**Repo root:** project root (where `Cargo.toml` lives) — do not create a nested project folder.

## Read order

| # | File | When |
|---|------|------|
| 1 | [STATUS.md](./STATUS.md) | Progress |
| 2 | [docs/DEEP_RESEARCH.md](./docs/DEEP_RESEARCH.md) | Pitfalls, versions, references |
| 3 | This file | Phases, APIs, done criteria |
| 4 | [docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md) | Layout |
| 5 | [docs/THREAT_MODEL.md](./docs/THREAT_MODEL.md) | Before claiming security |
| 6 | [docs/BLEND_INTEGRATION.md](./docs/BLEND_INTEGRATION.md) | Phase 5 |

## Autonomous mode

- Run until **Definition of Done** — do not stop after Phase 1.
- On **first** unrecognized error: web search (`stellar soroban` + error text). Do not guess.
- **Commit** after each phase verify passes ([Git commits](#git-commits)).
- Update [STATUS.md](./STATUS.md) in the same commit as phase completion.

## Git commits

Short imperative subject (≤72 chars). One logical change per commit.

**Do:** `Add SEP-40 circuit_breaker wrapper` · `Fix Symbol validation in tests`

**Don't:** `Update files` · `feat: stuff` · `Various fixes` · emoji

```bash
git add -A && git commit -m "Subject"
```

Never commit `target/`, `.deps/`, `test_snapshots/`, secrets.

## Out of scope

- Dashboards, indexers, explorers, protocol monitors
- Drips GitHub issues until repo accepted ([docs/WAVE.md](./docs/WAVE.md))
- GM-TWAP, multi-source quorum (v1)
- Mainnet deploy (testnet optional Phase 6)

## Deliverables

1. `contracts/mock_feed` — SEP-40 test oracle
2. `contracts/circuit_breaker` — staleness + deviation wrapper (SEP-40)
3. `contracts/twap_oracle` — AM-TWAP SEP-40 adapter
4. `crates/oracle-safety` — config types (extend as needed)
5. `packages/oracle-safety-ts` — Blend `PoolOracle` check script (pnpm)
6. `docs/*`, green `make verify`, README with deploy notes

---

## Phase 0 — Bootstrap (mostly done)

Scaffold exists. Verify:

```bash
make setup    # installs stellar-cli, pnpm, wasm target
cargo test --workspace
```

If `sep-40-oracle` / `soroban-sdk` version conflict: web search and align `[workspace.dependencies]` in root `Cargo.toml`.

---

## Phase 1 — `mock_feed` (SEP-40 test oracle)

**Goal:** Admin-controlled SEP-40 feed for tests.

Implement `PriceFeedTrait` from `sep-40-oracle` (see `.deps/sep-40-oracle` after `make setup` or docs.rs).

**Functions:**

| Function | Auth | Behavior |
|----------|------|----------|
| `initialize(admin, base, decimals, resolution)` | — | Once |
| `set_price(asset, price, timestamp)` | admin | Store price point |
| Trait methods | — | `base`, `assets`, `decimals`, `resolution`, `price`, `prices`, `lastprice` per SEP-40 |

**Tests** (`src/test.rs`):

- set price → `lastprice` returns it
- unknown asset → `None`
- stale timestamp rejected by consumers (document only)

**Verify:**

```bash
cargo test -p mock-feed
stellar contract build --package mock-feed
```

Remove `version()` placeholder when done.

---

## Phase 2 — `circuit_breaker`

**Goal:** SEP-40 wrapper around `source: Address`.

**Storage:** `Source`, `Admin`, `Config { max_staleness_secs, max_deviation_bps }`, `LastPrice: Map<Asset, PriceData>`

**Constructor:** `initialize(admin, source, config)`

**Admin:** `set_config(config)` — admin only

**Trait:** Implement `PriceFeedTrait` by delegating to `PriceFeedClient::new(env, &source)` then:

1. If `ledger_timestamp - price.timestamp > max_staleness_secs` → `None`
2. If previous price exists and bps deviation \> `max_deviation_bps` → `None`  
   `bps = abs(new-old)*10000/old` (integer math, handle `old==0`)
3. Update `LastPrice`, return price

Emit event `brk` on trip (optional, use short Symbol names).

**Tests:**

- fresh price passes
- stale → `None`
- 10% jump with 500 bps cap → `None`
- pass-through `decimals` / `base` from source

**Verify:**

```bash
cargo test -p circuit-breaker
stellar contract build --package circuit-breaker
```

---

## Phase 3 — `twap_oracle`

**Goal:** SEP-40 adapter; `lastprice` = AM-TWAP over last `periods` ticks from source.

**Storage:** `Source`, `Admin`, `Periods: u32`

**Constructor:** `initialize(admin, source, periods)`

**Admin:** `set_periods(periods)`

**`lastprice(asset)`:**

1. `let hist = source_client.prices(&asset, periods)?`
2. If `hist.len() < periods` → `None` (strict v0)
3. Reject any point older than ledger - `periods * resolution()` (or use circuit config from docs)
4. `price = sum(prices) / count` (i128, use source decimals)
5. Return `PriceData { price, timestamp: ledger_timestamp }`

Delegate `base`, `assets`, `decimals`, `resolution` to source.

**Tests:** mock_feed with monotonic prices → TWAP equals expected mean; insufficient history → `None`.

**Verify:**

```bash
cargo test -p twap-oracle
stellar contract build --package twap-oracle
```

---

## Phase 4 — Integration tests

Add `contracts/twap_oracle/tests/integration.rs` (or `src/integration_test.rs`):

1. Deploy mock_feed → twap_oracle → circuit_breaker (chain)
2. `lastprice` end-to-end
3. Trip breaker on spike

**Verify:**

```bash
cargo test -p twap-oracle --test integration
cargo test --workspace
stellar contract build
```

---

## Phase 5 — TypeScript (pnpm)

In `packages/oracle-safety-ts`:

1. `pnpm install` at repo root
2. Implement `src/examples/check-blend-oracle.ts` per [docs/BLEND_INTEGRATION.md](./docs/BLEND_INTEGRATION.md)
3. Read `ORACLE_ID` and `ASSETS` from env vars; document in README

**Verify:**

```bash
pnpm --filter @soroban-oracle-safety/ts run build
```

---

## Phase 6 — Polish

1. README: install, build, test, testnet deploy commands, **not audited**
2. `make verify` green
3. Optional: `scripts/deploy-testnet.sh` — deploy mock + twap + breaker, print contract IDs to `configs/deployed.testnet.json` (gitignored)
4. Update STATUS.md all phases done

**Verify:**

```bash
make verify
```

---

## Definition of Done

- [ ] All three contracts implement SEP-40 `PriceFeedTrait` (except mock is source)
- [ ] `cargo test --workspace` passes
- [ ] `stellar contract build` succeeds
- [ ] Integration test: mock → twap → breaker
- [ ] `pnpm` TS package builds
- [ ] README + THREAT_MODEL + ARCHITECTURE accurate
- [ ] No dashboards/monitors in repo
- [ ] **No** Drips GitHub issues created
- [ ] Commits: short, phase-scoped, no slop

---

## File map

```
contracts/mock_feed/src/{lib.rs,test.rs}
contracts/circuit_breaker/src/{lib.rs,test.rs,storage.rs?}
contracts/twap_oracle/src/{lib.rs,test.rs}
contracts/twap_oracle/tests/integration.rs
crates/oracle-safety/src/lib.rs
packages/oracle-safety-ts/src/
scripts/{setup-dev.sh,verify-all.sh}
```

Reference implementations: `.deps/oracle-aggregator`, `.deps/sep-40-oracle` (cloned by setup).

---

## Blocker protocol

1. Exact error → web search  
2. Fix with cited approach  
3. Note in commit or DEEP_RESEARCH changelog  
4. Never guess crate versions
