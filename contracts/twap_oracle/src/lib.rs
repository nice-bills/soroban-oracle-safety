//! SEP-40 TWAP adapter over a source oracle (AM-TWAP over `prices()` history).
//! Replace placeholder in Phase 3 per AGENT.md.

#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol};

#[contract]
pub struct TwapOracle;

#[contractimpl]
impl TwapOracle {
    pub fn version(env: Env) -> Symbol {
        Symbol::new(&env, "v0_ph")
    }
}

#[cfg(test)]
mod test;
