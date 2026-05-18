#![cfg(test)]

use mock_feed::{MockFeed, MockFeedClient};
use sep_40_oracle::Asset;
use soroban_sdk::{testutils::Address as _, testutils::Ledger, Address, Env};

use crate::{CircuitBreaker, CircuitBreakerClient, CircuitBreakerConfig};

fn deploy_mock(env: &Env, admin: &Address) -> Address {
    let base = Asset::Other(soroban_sdk::symbol_short!("USD"));
    let id = env.register(MockFeed, ());
    let client = MockFeedClient::new(env, &id);
    client.initialize(admin, &base, &7, &60);
    id
}

fn deploy_breaker<'a>(
    env: &'a Env,
    admin: &'a Address,
    source: &'a Address,
    config: CircuitBreakerConfig,
) -> CircuitBreakerClient<'a> {
    let id = env.register(CircuitBreaker, ());
    let client = CircuitBreakerClient::new(env, &id);
    client.initialize(admin, source, &config);
    client
}

#[test]
fn fresh_price_passes() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let mock_id = deploy_mock(&env, &admin);
    let mock = MockFeedClient::new(&env, &mock_id);
    let asset = Asset::Stellar(Address::generate(&env));

    let ts = env.ledger().timestamp();
    mock.set_price(&admin, &asset, &1_000_000i128, &ts);

    let breaker = deploy_breaker(
        &env,
        &admin,
        &mock_id,
        CircuitBreakerConfig {
            max_staleness_secs: 300,
            max_deviation_bps: 500,
        },
    );

    let out = breaker.lastprice(&asset).unwrap();
    assert_eq!(out.price, 1_000_000);
}

#[test]
fn stale_price_returns_none() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let mock_id = deploy_mock(&env, &admin);
    let mock = MockFeedClient::new(&env, &mock_id);
    let asset = Asset::Stellar(Address::generate(&env));

    mock.set_price(&admin, &asset, &1_000_000i128, &100);

    env.ledger().set_timestamp(10_000);

    let breaker = deploy_breaker(
        &env,
        &admin,
        &mock_id,
        CircuitBreakerConfig {
            max_staleness_secs: 60,
            max_deviation_bps: 500,
        },
    );

    assert!(breaker.lastprice(&asset).is_none());
}

#[test]
fn large_jump_trips_breaker() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let mock_id = deploy_mock(&env, &admin);
    let mock = MockFeedClient::new(&env, &mock_id);
    let asset = Asset::Stellar(Address::generate(&env));
    let ts = env.ledger().timestamp();

    mock.set_price(&admin, &asset, &1_000_000i128, &ts);

    let breaker = deploy_breaker(
        &env,
        &admin,
        &mock_id,
        CircuitBreakerConfig {
            max_staleness_secs: 300,
            max_deviation_bps: 500,
        },
    );
    breaker.lastprice(&asset).unwrap();

    mock.set_price(&admin, &asset, &1_200_000i128, &(ts + 60));

    assert!(breaker.lastprice(&asset).is_none());
}

#[test]
fn passthrough_metadata_from_source() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let mock_id = deploy_mock(&env, &admin);
    let breaker = deploy_breaker(&env, &admin, &mock_id, CircuitBreakerConfig::default());
    let mock = MockFeedClient::new(&env, &mock_id);

    assert_eq!(breaker.decimals(), mock.decimals());
    assert_eq!(breaker.decimals(), 7);
    assert_eq!(breaker.resolution(), mock.resolution());
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            max_staleness_secs: 300,
            max_deviation_bps: 500,
        }
    }
}
