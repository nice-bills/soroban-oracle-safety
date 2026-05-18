# Blend integration

## Validate oracle for a pool

After deploying `twap_oracle` or `circuit_breaker` on testnet:

```typescript
import { PoolOracle } from "@blend-capital/blend-sdk";

const network = {
  passphrase: "Test SDF Network ; September 2015",
  rpc: "https://soroban-testnet.stellar.org",
};

const oracle_id = "C..."; // deployed adapter contract id
const assets = ["C...", "C..."]; // reserve SAC contract ids

const oracle = await PoolOracle.load(network, oracle_id, assets);
console.log(JSON.stringify(oracle, null, 2));
```

Implement in `packages/oracle-safety-ts/src/examples/check-blend-oracle.ts` (Phase 5).

## Pool creator checklist

1. Adapter implements SEP-40 `lastprice` + `decimals`.
2. Test every reserve asset with `PoolOracle.load`.
3. Prefer TWAP + circuit breaker for volatile reserves.
4. Remember: **oracle address is immutable** after pool creation.

Reference: https://docs-v1.blend.capital/pool-creators/selecting-an-oracle
