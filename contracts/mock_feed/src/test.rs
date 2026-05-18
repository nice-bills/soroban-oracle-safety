#![cfg(test)]

use sep_40_oracle::Asset;
use soroban_sdk::{symbol_short, testutils::Address as _, Address, Env};

use crate::{MockFeed, MockFeedClient};

fn setup(env: &Env) -> (Address, MockFeedClient<'_>) {
    let admin = Address::generate(env);
    let base = Asset::Other(symbol_short!("USD"));
    let contract_id = env.register(MockFeed, ());
    let client = MockFeedClient::new(env, &contract_id);
    env.mock_all_auths();
    client.initialize(&admin, &base, &7, &60);
    (admin, client)
}

#[test]
fn set_price_lastprice_returns_it() {
    let env = Env::default();
    let (admin, client) = setup(&env);
    let asset = Asset::Stellar(Address::generate(&env));
    let ts = 1_000u64;

    client.set_price(&admin, &asset, &1_500_000_000i128, &ts);

    let last = client.lastprice(&asset).unwrap();
    assert_eq!(last.price, 1_500_000_000);
    assert_eq!(last.timestamp, 960); // normalized to 60s resolution
}

#[test]
fn unknown_asset_returns_none() {
    let env = Env::default();
    let (_admin, client) = setup(&env);
    let unknown = Asset::Stellar(Address::generate(&env));
    assert!(client.lastprice(&unknown).is_none());
}

#[test]
fn prices_history_walks_back() {
    let env = Env::default();
    let (admin, client) = setup(&env);
    let asset = Asset::Stellar(Address::generate(&env));

    client.set_price(&admin, &asset, &100i128, &120);
    client.set_price(&admin, &asset, &200i128, &180);
    client.set_price(&admin, &asset, &300i128, &240);

    let hist = client.prices(&asset, &3).unwrap();
    assert_eq!(hist.len(), 3);
    assert_eq!(hist.get(0).unwrap().price, 300);
    assert_eq!(hist.get(1).unwrap().price, 200);
    assert_eq!(hist.get(2).unwrap().price, 100);
}

#[test]
fn trait_metadata() {
    let env = Env::default();
    let (admin, client) = setup(&env);
    match (client.base(), Asset::Other(symbol_short!("USD"))) {
        (Asset::Other(a), Asset::Other(b)) => assert_eq!(a, b),
        _ => panic!("expected USD base"),
    }
    assert_eq!(client.decimals(), 7);
    assert_eq!(client.resolution(), 60);

    let asset = Asset::Stellar(Address::generate(&env));
    client.set_price(&admin, &asset, &42i128, &60);
    let assets = client.assets();
    assert_eq!(assets.len(), 1);
}
