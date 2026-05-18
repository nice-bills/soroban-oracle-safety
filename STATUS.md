# Status

| Phase | State | Verify |
|-------|-------|--------|
| 0 Bootstrap | done | `cargo test --workspace` |
| 1 mock_feed | done | `cargo test -p mock-feed` |
| 2 circuit_breaker | done | `cargo test -p circuit-breaker` |
| 3 twap_oracle | done | `cargo test -p twap-oracle` |
| 4 integration | done | `cargo test -p twap-oracle --test integration` |
| 5 TypeScript | done | `pnpm --filter @soroban-oracle-safety/ts run build` |
| 6 docs + CI | done | `make verify` |

**Blockers:** none

**Wave:** repo not applied yet — do not create GitHub issues ([docs/WAVE.md](./docs/WAVE.md)).
