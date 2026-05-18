# soroban-oracle-safety

Open-source **SEP-40 oracle safety adapters** for Stellar Soroban: staleness checks, deviation circuit breakers, and AM-TWAP — for Blend and other DeFi pools.

Not a dashboard. Not an indexer.

## For agents

**Repo root:** `/home/bills/code/soroban-oracle-safety`

1. Read [AGENT.md](./AGENT.md)
2. Paste [PROMPT.md](./PROMPT.md) into a new agent chat
3. Run `make setup` then implement Phases 1–6

## Quick start (after implementation)

```bash
cd /home/bills/code/soroban-oracle-safety
make setup
make verify
```

## Docs

- [Architecture](./docs/ARCHITECTURE.md)
- [Threat model](./docs/THREAT_MODEL.md)
- [Deep research](./docs/DEEP_RESEARCH.md)
- [Blend integration](./docs/BLEND_INTEGRATION.md)
- [Drips Wave (later)](./docs/WAVE.md)

## License

MIT — see [LICENSE](./LICENSE). **Not audited.**
