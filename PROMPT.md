# One-shot prompt

Open workspace: `/home/bills/code/soroban-oracle-safety`

---

```
Implement soroban-oracle-safety from 0% to 100% per AGENT.md in this directory.

READ FIRST: STATUS.md → docs/DEEP_RESEARCH.md → AGENT.md → docs/ARCHITECTURE.md

RULES:
- Repo root = /home/bills/code/soroban-oracle-safety (no nested subfolder)
- Do NOT create Drips/Wave GitHub issues (repo not accepted yet)
- On first unknown error: web search; do not guess versions or APIs
- TypeScript: pnpm only. Python: not used in v0
- Commit after each phase passes verify; short imperative messages (see AGENT.md)
- Out of scope: dashboards, indexers, explorers, TVL monitors

PHASES: 1 mock_feed → 2 circuit_breaker → 3 twap_oracle → 4 integration → 5 TS → 6 polish

DONE when AGENT.md Definition of Done passes and `make verify` is green.
```
