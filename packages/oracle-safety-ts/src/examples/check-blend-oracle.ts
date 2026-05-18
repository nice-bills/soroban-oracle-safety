/**
 * Validate a deployed SEP-40 oracle adapter against Blend reserve assets.
 *
 * Env:
 *   ORACLE_ID  — contract id (C...)
 *   ASSETS     — comma-separated reserve SAC contract ids
 *   RPC_URL    — optional, defaults to testnet
 *   PASSPHRASE — optional, defaults to testnet
 */
import { PoolOracle } from "@blend-capital/blend-sdk";

const DEFAULT_PASSPHRASE = "Test SDF Network ; September 2015";
const DEFAULT_RPC = "https://soroban-testnet.stellar.org";

function requireEnv(name: string): string {
  const value = process.env[name]?.trim();
  if (!value) {
    console.error(`Missing required env var: ${name}`);
    process.exit(1);
  }
  return value;
}

async function main(): Promise<void> {
  const oracle_id = requireEnv("ORACLE_ID");
  const assetsRaw = requireEnv("ASSETS");
  const assets = assetsRaw
    .split(",")
    .map((s) => s.trim())
    .filter(Boolean);

  if (assets.length === 0) {
    console.error("ASSETS must list at least one reserve contract id");
    process.exit(1);
  }

  const network = {
    passphrase: process.env.PASSPHRASE?.trim() || DEFAULT_PASSPHRASE,
    rpc: process.env.RPC_URL?.trim() || DEFAULT_RPC,
  };

  console.log(`Loading PoolOracle for ${oracle_id} (${assets.length} assets)...`);
  const oracle = await PoolOracle.load(network, oracle_id, assets);
  console.log(JSON.stringify(oracle, null, 2));
}

main().catch((err: unknown) => {
  console.error(err);
  process.exit(1);
});
