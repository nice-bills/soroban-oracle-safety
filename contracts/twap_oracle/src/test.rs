#![cfg(test)]

use oracle_test_utils::deploy_mock;
use sep_40_oracle::Asset;
use soroban_sdk::{testutils::Address as _, Address, Env};

use crate::{TwapOracle, TwapOracleClient};

#[test]
fn twap_equals_arithmetic_mean() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let (mock_id, mock) = deploy_mock(&env, &admin);
    let asset = Asset::Stellar(Address::generate(&env));

    mock.set_price(&admin, &asset, &100i128, &60);
    mock.set_price(&admin, &asset, &200i128, &120);
    mock.set_price(&admin, &asset, &300i128, &180);

    let twap_id = env.register(TwapOracle, ());
    let twap = TwapOracleClient::new(&env, &twap_id);
    twap.initialize(&admin, &mock_id, &3);

    let out = twap.lastprice(&asset).unwrap();
    assert_eq!(out.price, 200);
}

#[test]
fn insufficient_history_returns_none() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let (mock_id, mock) = deploy_mock(&env, &admin);
    let asset = Asset::Stellar(Address::generate(&env));

    mock.set_price(&admin, &asset, &100i128, &60);
    mock.set_price(&admin, &asset, &200i128, &120);

    let twap_id = env.register(TwapOracle, ());
    let twap = TwapOracleClient::new(&env, &twap_id);
    twap.initialize(&admin, &mock_id, &3);

    assert!(twap.lastprice(&asset).is_none());
}

#[test]
fn delegates_metadata() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let (mock_id, mock) = deploy_mock(&env, &admin);

    let twap_id = env.register(TwapOracle, ());
    let twap = TwapOracleClient::new(&env, &twap_id);
    twap.initialize(&admin, &mock_id, &5);

    assert_eq!(twap.decimals(), mock.decimals());
    assert_eq!(twap.resolution(), mock.resolution());
}
