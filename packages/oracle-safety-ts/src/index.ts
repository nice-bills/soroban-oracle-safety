/**
 * TypeScript helpers for validating SEP-40 oracle adapters (Blend PoolOracle pattern).
 * Implement fully in Phase 5 per AGENT.md.
 */

export type NetworkConfig = {
  passphrase: string;
  rpc: string;
};

export const TESTNET: NetworkConfig = {
  passphrase: "Test SDF Network ; September 2015",
  rpc: "https://soroban-testnet.stellar.org",
};
