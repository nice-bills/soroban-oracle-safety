//! SEP-40 mock price feed for unit and integration tests.

#![no_std]

mod storage;

use sep_40_oracle::{Asset, PriceData, PriceFeedTrait};
use soroban_sdk::{contract, contractimpl, Address, Env, Vec};

#[contract]
pub struct MockFeed;

#[contractimpl]
impl MockFeed {
    /// Initialize the mock feed (once).
    pub fn initialize(env: Env, admin: Address, base: Asset, decimals: u32, resolution: u32) {
        if storage::has_admin(&env) {
            panic!("already initialized");
        }
        admin.require_auth();
        storage::set_admin(&env, &admin);
        storage::set_base(&env, &base);
        storage::set_decimals(&env, decimals);
        storage::set_resolution(&env, resolution);
        storage::set_assets(&env, &Vec::new(&env));
        storage::extend_instance(&env);
    }

    /// Admin sets a price point for an asset at a timestamp (normalized to resolution).
    pub fn set_price(env: Env, admin: Address, asset: Asset, price: i128, timestamp: u64) {
        admin.require_auth();
        if admin != storage::get_admin(&env) {
            panic!("not admin");
        }
        let resolution = storage::get_resolution(&env) as u64;
        let normalized = normalize_timestamp(timestamp, resolution);
        storage::set_price(&env, &asset, normalized, price);
        storage::set_last_timestamp(&env, &asset, normalized);
        add_asset_if_new(&env, &asset);
        storage::extend_instance(&env);
    }
}

#[contractimpl]
impl PriceFeedTrait for MockFeed {
    fn base(env: Env) -> Asset {
        storage::get_base(&env)
    }

    fn assets(env: Env) -> Vec<Asset> {
        storage::get_assets(&env)
    }

    fn decimals(env: Env) -> u32 {
        storage::get_decimals(&env)
    }

    fn resolution(env: Env) -> u32 {
        storage::get_resolution(&env)
    }

    fn price(env: Env, asset: Asset, timestamp: u64) -> Option<PriceData> {
        let resolution = storage::get_resolution(&env) as u64;
        if resolution == 0 {
            return None;
        }
        let normalized = normalize_timestamp(timestamp, resolution);
        storage::get_price(&env, &asset, normalized).map(|price| PriceData {
            price,
            timestamp: normalized,
        })
    }

    fn prices(env: Env, asset: Asset, records: u32) -> Option<Vec<PriceData>> {
        let resolution = storage::get_resolution(&env) as u64;
        let mut timestamp = storage::get_last_timestamp(&env, &asset)?;
        if resolution == 0 {
            return None;
        }

        let mut out: Vec<PriceData> = Vec::new(&env);
        let limit = if records > 20 { 20 } else { records };
        for _ in 0..limit {
            if let Some(price) = storage::get_price(&env, &asset, timestamp) {
                out.push_back(PriceData { price, timestamp });
            } else {
                break;
            }
            if timestamp < resolution {
                break;
            }
            timestamp -= resolution;
        }
        if out.is_empty() {
            None
        } else {
            Some(out)
        }
    }

    fn lastprice(env: Env, asset: Asset) -> Option<PriceData> {
        let timestamp = storage::get_last_timestamp(&env, &asset)?;
        let price = storage::get_price(&env, &asset, timestamp)?;
        Some(PriceData { price, timestamp })
    }
}

fn normalize_timestamp(timestamp: u64, resolution: u64) -> u64 {
    if resolution == 0 {
        return timestamp;
    }
    let bucket = timestamp.checked_div(resolution).unwrap_or(0);
    bucket.saturating_mul(resolution)
}

fn add_asset_if_new(env: &Env, asset: &Asset) {
    let mut assets = storage::get_assets(env);
    for existing in assets.iter() {
        if assets_equal(&existing, asset) {
            return;
        }
    }
    assets.push_back(asset.clone());
    storage::set_assets(env, &assets);
}

fn assets_equal(a: &Asset, b: &Asset) -> bool {
    match (a, b) {
        (Asset::Stellar(x), Asset::Stellar(y)) => x == y,
        (Asset::Other(x), Asset::Other(y)) => x == y,
        _ => false,
    }
}

#[cfg(test)]
mod test;
