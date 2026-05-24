#![cfg(test)]

use mock_feed::{MockFeed, MockFeedClient};
use sep_40_oracle::Asset;
use soroban_sdk::{symbol_short, testutils::Address as _, testutils::Ledger, Address, Env};

use crate::{CircuitBreaker, CircuitBreakerClient, CircuitBreakerConfig};

fn deploy_mock<'a>(env: &'a Env, admin: &Address) -> (Address, MockFeedClient<'a>) {
    let base = Asset::Other(symbol_short!("USD"));
    let id = env.register(MockFeed, ());
    let client = MockFeedClient::new(env, &id);
    client.initialize(admin, &base, &7, &60);
    (id, client)
}

fn deploy_breaker<'a>(
    env: &'a Env,
    admin: &Address,
    source: &Address,
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
    let (mock_id, mock) = deploy_mock(&env, &admin);
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
    let (mock_id, mock) = deploy_mock(&env, &admin);
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
    let (mock_id, mock) = deploy_mock(&env, &admin);
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
fn guarded_price_matches_lastprice() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let (mock_id, mock) = deploy_mock(&env, &admin);
    let asset = Asset::Stellar(Address::generate(&env));
    let ts = env.ledger().timestamp();

    mock.set_price(&admin, &asset, &500_000i128, &ts);

    let breaker = deploy_breaker(&env, &admin, &mock_id, CircuitBreakerConfig::default());
    let via_last = breaker.lastprice(&asset).unwrap();
    let via_price = breaker.price(&asset, &ts).unwrap();
    assert_eq!(via_last.price, via_price.price);
}

#[test]
fn passthrough_metadata_from_source() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let (mock_id, _) = deploy_mock(&env, &admin);
    let breaker = deploy_breaker(&env, &admin, &mock_id, CircuitBreakerConfig::default());

    assert_eq!(breaker.decimals(), 7);
    assert_eq!(breaker.resolution(), 60);
}

#[test]
fn zero_staleness_config_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let (mock_id, _) = deploy_mock(&env, &admin);

    let breaker_id = env.register(CircuitBreaker, ());
    let breaker = CircuitBreakerClient::new(&env, &breaker_id);
    let err = breaker
        .try_initialize(
            &admin,
            &mock_id,
            &CircuitBreakerConfig {
                max_staleness_secs: 0,
                max_deviation_bps: 500,
            },
        )
        .expect_err("expected error");
    assert_eq!(err, Ok(crate::error::AdapterError::InvalidConfig));
}
