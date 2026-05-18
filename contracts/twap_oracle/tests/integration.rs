//! End-to-end: mock_feed → twap_oracle → circuit_breaker.

use circuit_breaker::{CircuitBreaker, CircuitBreakerClient, CircuitBreakerConfig};
use mock_feed::{MockFeed, MockFeedClient};
use sep_40_oracle::Asset;
use soroban_sdk::{testutils::Address as _, Address, Env};
use twap_oracle::{TwapOracle, TwapOracleClient};

#[test]
fn chain_lastprice_end_to_end() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);

    let base = Asset::Other(soroban_sdk::symbol_short!("USD"));
    let mock_id = env.register(MockFeed, ());
    let mock = MockFeedClient::new(&env, &mock_id);
    mock.initialize(&admin, &base, &7, &60);

    let asset = Asset::Stellar(Address::generate(&env));
    mock.set_price(&admin, &asset, &100i128, &60);
    mock.set_price(&admin, &asset, &200i128, &120);
    mock.set_price(&admin, &asset, &300i128, &180);

    let twap_id = env.register(TwapOracle, ());
    let twap = TwapOracleClient::new(&env, &twap_id);
    twap.initialize(&admin, &mock_id, &3);

    let breaker_id = env.register(CircuitBreaker, ());
    let breaker = CircuitBreakerClient::new(&env, &breaker_id);
    breaker.initialize(
        &admin,
        &twap_id,
        &CircuitBreakerConfig {
            max_staleness_secs: 600,
            max_deviation_bps: 500,
        },
    );

    let price = breaker.lastprice(&asset).unwrap();
    assert_eq!(price.price, 200);
}

#[test]
fn chain_trips_breaker_on_spike() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);

    let base = Asset::Other(soroban_sdk::symbol_short!("USD"));
    let mock_id = env.register(MockFeed, ());
    let mock = MockFeedClient::new(&env, &mock_id);
    mock.initialize(&admin, &base, &7, &60);

    let asset = Asset::Stellar(Address::generate(&env));
    let ts = env.ledger().timestamp();
    mock.set_price(&admin, &asset, &1_000_000i128, &ts);

    let twap_id = env.register(TwapOracle, ());
    let twap = TwapOracleClient::new(&env, &twap_id);
    twap.initialize(&admin, &mock_id, &1);

    let breaker_id = env.register(CircuitBreaker, ());
    let breaker = CircuitBreakerClient::new(&env, &breaker_id);
    breaker.initialize(
        &admin,
        &twap_id,
        &CircuitBreakerConfig {
            max_staleness_secs: 600,
            max_deviation_bps: 500,
        },
    );
    breaker.lastprice(&asset).unwrap();

    mock.set_price(&admin, &asset, &2_000_000i128, &(ts + 60));

    assert!(breaker.lastprice(&asset).is_none());
}
