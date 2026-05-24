//! SEP-40 oracle wrapper: staleness and max-deviation circuit breaker.
//!
//! All `PriceFeedTrait` methods apply guards (staleness + deviation on cached prior).

#![no_std]

mod error;
mod policy;
mod storage;

use error::AdapterError;
use sep_40_oracle::{Asset, PriceData, PriceFeedClient, PriceFeedTrait};
use soroban_sdk::{contract, contractimpl, panic_with_error, Address, Env, Vec};

pub use storage::CircuitBreakerConfig;

#[contract]
pub struct CircuitBreaker;

#[contractimpl]
impl CircuitBreaker {
    pub fn initialize(
        env: Env,
        admin: Address,
        source: Address,
        config: CircuitBreakerConfig,
    ) -> Result<(), AdapterError> {
        if storage::has_admin(&env) {
            return Err(AdapterError::AlreadyInitialized);
        }
        config.validate().map_err(AdapterError::from)?;
        admin.require_auth();
        storage::set_admin(&env, &admin);
        storage::set_source(&env, &source);
        storage::set_config(&env, &config);
        storage::extend_instance(&env);
        Ok(())
    }

    pub fn set_config(
        env: Env,
        admin: Address,
        config: CircuitBreakerConfig,
    ) -> Result<(), AdapterError> {
        admin.require_auth();
        storage::require_admin(&env, &admin).map_err(AdapterError::from)?;
        config.validate().map_err(AdapterError::from)?;
        storage::set_config(&env, &config);
        storage::extend_instance(&env);
        Ok(())
    }
}

#[contractimpl]
impl PriceFeedTrait for CircuitBreaker {
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
        policy::guarded_price(&env, &source_client(&env), &asset, timestamp)
    }

    fn prices(env: Env, asset: Asset, records: u32) -> Option<Vec<PriceData>> {
        policy::guarded_prices(&env, &source_client(&env), &asset, records)
    }

    fn lastprice(env: Env, asset: Asset) -> Option<PriceData> {
        policy::guarded_lastprice(&env, &source_client(&env), &asset)
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
