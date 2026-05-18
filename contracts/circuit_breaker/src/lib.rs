//! SEP-40 oracle wrapper: staleness and max-deviation circuit breaker.

#![no_std]

mod storage;

use sep_40_oracle::{Asset, PriceData, PriceFeedClient, PriceFeedTrait};
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, Vec};

pub use storage::CircuitBreakerConfig;

#[contract]
pub struct CircuitBreaker;

#[contractimpl]
impl CircuitBreaker {
    pub fn initialize(env: Env, admin: Address, source: Address, config: CircuitBreakerConfig) {
        if storage::has_admin(&env) {
            panic!("already initialized");
        }
        admin.require_auth();
        storage::set_admin(&env, &admin);
        storage::set_source(&env, &source);
        storage::set_config(&env, &config);
        storage::extend_instance(&env);
    }

    pub fn set_config(env: Env, admin: Address, config: CircuitBreakerConfig) {
        admin.require_auth();
        if admin != storage::get_admin(&env) {
            panic!("not admin");
        }
        storage::set_config(&env, &config);
        storage::extend_instance(&env);
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
        source_client(&env).price(&asset, &timestamp)
    }

    fn prices(env: Env, asset: Asset, records: u32) -> Option<Vec<PriceData>> {
        source_client(&env).prices(&asset, &records)
    }

    fn lastprice(env: Env, asset: Asset) -> Option<PriceData> {
        let config = storage::get_config(&env);
        let inner = source_client(&env).lastprice(&asset)?;

        let now = env.ledger().timestamp();
        if now.saturating_sub(inner.timestamp) > config.max_staleness_secs {
            env.events()
                .publish((Symbol::new(&env, "brk"),), ("stale", asset.clone()));
            return None;
        }

        if let Some(prev) = storage::get_last_price(&env, &asset) {
            if exceeds_deviation(prev.price, inner.price, config.max_deviation_bps) {
                env.events()
                    .publish((Symbol::new(&env, "brk"),), ("dev", asset.clone()));
                return None;
            }
        }

        storage::set_last_price(&env, &asset, &inner);
        Some(inner)
    }
}

fn source_client(env: &Env) -> PriceFeedClient<'_> {
    PriceFeedClient::new(env, &storage::get_source(env))
}

/// `bps = abs(new - old) * 10000 / old` (old == 0 skips deviation check).
fn exceeds_deviation(old: i128, new: i128, max_bps: u32) -> bool {
    if max_bps == 0 || old == 0 {
        return false;
    }
    let diff = if new >= old { new - old } else { old - new };
    let bps = diff.saturating_mul(10_000) / old.abs();
    bps > max_bps as i128
}

#[cfg(test)]
mod test;
