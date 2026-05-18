#![cfg(test)]

use super::*;
use soroban_sdk::Env;

#[test]
fn placeholder_compiles() {
    let env = Env::default();
    let id = env.register(TwapOracle, ());
    let client = TwapOracleClient::new(&env, &id);
    let _ = client.version();
}
