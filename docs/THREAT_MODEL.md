# Threat model (v0)

## In scope mitigations

| Threat | Mitigation |
|--------|------------|
| Stale oracle price | `max_staleness_secs` in circuit_breaker |
| Single-block spot spike | AM-TWAP over `N` periods |
| Large jump vs previous tick | `max_deviation_bps` in circuit_breaker (all SEP-40 reads) |
| Wrong asset queried | SEP-40 `None` — consumer must handle |
| Unbounded `prices()` history | `mock_feed` caps at `MAX_PRICE_RECORDS` (20) |

## Architecture notes

- **`oracle-storage`**: shared admin/source/TTL helpers for adapter contracts.
- **`circuit_breaker`**: per-asset `LastP` keys (not one map for all assets); guards apply to `lastprice`, `price`, and `prices` (newest point for `prices`).
- **Chain**: `mock_feed` → `twap_oracle` → `circuit_breaker` (recommended).

## Out of scope / honest limits

- **`max_deviation_bps = 0`**: deviation check disabled (staleness still applies).
- **Compromised upstream oracle**: we trust the source SEP-40 feed; no multi-source quorum in v0.
- **Admin key compromise**: admin can change config on wrappers.
- **Illiquid assets**: oracles for thin markets remain unsafe regardless of adapter.
- **Governance / upgrade**: contracts should use Soroban upgrade pattern if deployed mainnet (document in README).

## Post-Blend context

Feb 2026 Blend exploit involved oracle manipulation. This kit helps pool creators adopt **TWAP + staleness + deviation** policies; it is **not** a substitute for audit, monitoring, or economic risk review.

## Audit status

**Not audited.** Mark prominently in README until audit completes.
