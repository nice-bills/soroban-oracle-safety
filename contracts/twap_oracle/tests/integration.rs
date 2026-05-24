//! End-to-end: mock_feed → twap_oracle → circuit_breaker.

use oracle_test_utils::{deploy_chain, CircuitBreakerConfig};
use sep_40_oracle::Asset;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn chain_lastprice_end_to_end() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);

    let chain = deploy_chain(
        &env,
        &admin,
        3,
        CircuitBreakerConfig {
            max_staleness_secs: 600,
            max_deviation_bps: 500,
        },
    );

    let asset = Asset::Stellar(Address::generate(&env));
    chain.mock.set_price(&admin, &asset, &100i128, &60);
    chain.mock.set_price(&admin, &asset, &200i128, &120);
    chain.mock.set_price(&admin, &asset, &300i128, &180);

    let price = chain.breaker.lastprice(&asset).unwrap();
    assert_eq!(price.price, 200);
}

#[test]
fn chain_trips_breaker_on_spike() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);

    let chain = deploy_chain(
        &env,
        &admin,
        1,
        CircuitBreakerConfig {
            max_staleness_secs: 600,
            max_deviation_bps: 500,
        },
    );

    let asset = Asset::Stellar(Address::generate(&env));
    let ts = env.ledger().timestamp();
    chain.mock.set_price(&admin, &asset, &1_000_000i128, &ts);

    chain.breaker.lastprice(&asset).unwrap();

    chain
        .mock
        .set_price(&admin, &asset, &2_000_000i128, &(ts + 60));

    assert!(chain.breaker.lastprice(&asset).is_none());
}
