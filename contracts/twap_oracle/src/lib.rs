//! SEP-40 TWAP adapter: AM-TWAP over source `prices()` history.

#![no_std]

mod storage;

use sep_40_oracle::{Asset, PriceData, PriceFeedClient, PriceFeedTrait};
use soroban_sdk::{contract, contractimpl, Address, Env, Vec};

#[contract]
pub struct TwapOracle;

#[contractimpl]
impl TwapOracle {
    pub fn initialize(env: Env, admin: Address, source: Address, periods: u32) {
        if storage::has_admin(&env) {
            panic!("already initialized");
        }
        admin.require_auth();
        storage::set_admin(&env, &admin);
        storage::set_source(&env, &source);
        storage::set_periods(&env, periods);
        storage::extend_instance(&env);
    }

    pub fn set_periods(env: Env, admin: Address, periods: u32) {
        admin.require_auth();
        if admin != storage::get_admin(&env) {
            panic!("not admin");
        }
        storage::set_periods(&env, periods);
        storage::extend_instance(&env);
    }
}

#[contractimpl]
impl PriceFeedTrait for TwapOracle {
    fn base(env: Env) -> Asset {
        source_client(&env).base()
    }

    fn assets(env: Env) -> Vec<Asset> {
        source_client(&env).assets()
    }

    fn decimals(env: Env) -> u32 {
        source_client(&env).decimals()
    }

    fn resolution(env: Env) -> u32 {
        source_client(&env).resolution()
    }

    fn price(env: Env, asset: Asset, timestamp: u64) -> Option<PriceData> {
        source_client(&env).price(&asset, &timestamp)
    }

    fn prices(env: Env, asset: Asset, records: u32) -> Option<Vec<PriceData>> {
        source_client(&env).prices(&asset, &records)
    }

    fn lastprice(env: Env, asset: Asset) -> Option<PriceData> {
        let periods = storage::get_periods(&env);
        let source = source_client(&env);
        let hist = source.prices(&asset, &periods)?;
        if hist.len() < periods {
            return None;
        }

        let resolution = source.resolution() as u64;
        let now = env.ledger().timestamp();
        let oldest_allowed = now.saturating_sub(periods as u64 * resolution);

        let mut sum: i128 = 0;
        for i in 0..hist.len() {
            let point = hist.get(i).unwrap();
            if point.timestamp < oldest_allowed {
                return None;
            }
            sum = sum.saturating_add(point.price);
        }

        let count = hist.len() as i128;
        let avg = sum / count;
        Some(PriceData {
            price: avg,
            timestamp: now,
        })
    }
}

fn source_client(env: &Env) -> PriceFeedClient<'_> {
    PriceFeedClient::new(env, &storage::get_source(env))
}

#[cfg(test)]
mod test;
