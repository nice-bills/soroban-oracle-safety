//! SEP-40 TWAP adapter: AM-TWAP over source `prices()` history.

#![no_std]

mod error;
mod storage;

use error::AdapterError;
use sep_40_oracle::{Asset, PriceData, PriceFeedClient, PriceFeedTrait};
use soroban_sdk::{contract, contractimpl, panic_with_error, Address, Env, Vec};

#[contract]
pub struct TwapOracle;

#[contractimpl]
impl TwapOracle {
    pub fn initialize(
        env: Env,
        admin: Address,
        source: Address,
        periods: u32,
    ) -> Result<(), AdapterError> {
        if storage::has_admin(&env) {
            return Err(AdapterError::AlreadyInitialized);
        }
        if periods == 0 {
            return Err(AdapterError::InvalidConfig);
        }
        admin.require_auth();
        storage::set_admin(&env, &admin);
        storage::set_source(&env, &source);
        storage::set_periods(&env, periods);
        storage::extend_instance(&env);
        Ok(())
    }

    pub fn set_periods(env: Env, admin: Address, periods: u32) -> Result<(), AdapterError> {
        admin.require_auth();
        storage::require_admin(&env, &admin).map_err(AdapterError::from)?;
        if periods == 0 {
            return Err(AdapterError::InvalidConfig);
        }
        storage::set_periods(&env, periods);
        storage::extend_instance(&env);
        Ok(())
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
        let len = hist.len();
        if len < periods {
            return None;
        }

        let resolution = source.resolution() as u64;
        let now = env.ledger().timestamp();
        let oldest_allowed = now.saturating_sub(periods as u64 * resolution);

        let mut sum: i128 = 0;
        for i in 0..len {
            let point = hist.get(i)?;
            if point.timestamp < oldest_allowed {
                return None;
            }
            sum = sum.saturating_add(point.price);
        }

        let avg = sum / (len as i128);
        Some(PriceData {
            price: avg,
            timestamp: now,
        })
    }
}

fn source_client(env: &Env) -> PriceFeedClient<'_> {
    match storage::get_source(env) {
        Ok(addr) => PriceFeedClient::new(env, &addr),
        Err(e) => panic_with_error!(env, AdapterError::from(e)),
    }
}

#[cfg(test)]
mod test;
