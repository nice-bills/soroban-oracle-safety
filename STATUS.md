# Status

| Phase | State | Verify |
|-------|-------|--------|
| 0 Bootstrap | done (scaffold) | `cargo test --workspace` |
| 1 mock_feed | pending | `cargo test -p mock-feed` |
| 2 circuit_breaker | pending | `cargo test -p circuit-breaker` |
| 3 twap_oracle | pending | `cargo test -p twap-oracle` |
| 4 integration | pending | `cargo test -p twap-oracle --test integration` |
| 5 TypeScript | pending | `pnpm --filter @soroban-oracle-safety/ts run build` |
| 6 docs + CI | pending | `make verify` |

**Blockers:** none

**Wave:** repo not applied yet — do not create GitHub issues ([docs/WAVE.md](./docs/WAVE.md)).

**Next agent:** read [AGENT.md](./AGENT.md), run `make setup`, implement Phases 1–6.
