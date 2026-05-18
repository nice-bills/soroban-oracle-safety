//! SEP-40 mock price feed for unit/integration tests.
//! Replace this placeholder in Phase 1 per AGENT.md.

#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol};

#[contract]
pub struct MockFeed;

#[contractimpl]
impl MockFeed {
    /// Placeholder — implement `PriceFeedTrait` via sep-40-oracle in Phase 1.
    pub fn version(env: Env) -> Symbol {
        Symbol::new(&env, "v0_ph")
    }
}

#[cfg(test)]
mod test;
