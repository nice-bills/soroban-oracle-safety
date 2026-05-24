//! Staleness and deviation checks for guarded SEP-40 reads.

use sep_40_oracle::{Asset, PriceData, PriceFeedClient};
use soroban_sdk::{Env, Symbol};

use crate::storage::{self, CircuitBreakerConfig};

pub fn guarded_lastprice(
    env: &Env,
    source: &PriceFeedClient<'_>,
    asset: &Asset,
) -> Option<PriceData> {
    let config = storage::get_config(env);
    let inner = source.lastprice(asset)?;
    apply_guards(env, asset, &config, inner)
}

pub fn guarded_price(
    env: &Env,
    source: &PriceFeedClient<'_>,
    asset: &Asset,
    timestamp: u64,
) -> Option<PriceData> {
    let config = storage::get_config(env);
    let inner = source.price(asset, &timestamp)?;
    apply_guards(env, asset, &config, inner)
}

pub fn guarded_prices(
    env: &Env,
    source: &PriceFeedClient<'_>,
    asset: &Asset,
    records: u32,
) -> Option<soroban_sdk::Vec<PriceData>> {
    let config = storage::get_config(env);
    let hist = source.prices(asset, &records)?;
    if hist.is_empty() {
        return None;
    }
    let newest = hist.get(0)?;
    apply_guards(env, asset, &config, newest)?;
    Some(hist)
}

fn apply_guards(
    env: &Env,
    asset: &Asset,
    config: &CircuitBreakerConfig,
    inner: PriceData,
) -> Option<PriceData> {
    let now = env.ledger().timestamp();
    if now.saturating_sub(inner.timestamp) > config.max_staleness_secs {
        env.events()
            .publish((Symbol::new(env, "brk"),), ("stale", asset.clone()));
        return None;
    }

    if let Some(prev) = storage::get_last_price(env, asset) {
        if exceeds_deviation(prev.price, inner.price, config.max_deviation_bps) {
            env.events()
                .publish((Symbol::new(env, "brk"),), ("dev", asset.clone()));
            return None;
        }
    }

    storage::set_last_price(env, asset, &inner);
    Some(inner)
}

/// `bps = abs(new - old) * 10000 / old` (old == 0 or max_bps == 0 skips deviation check).
fn exceeds_deviation(old: i128, new: i128, max_bps: u32) -> bool {
    if max_bps == 0 || old == 0 {
        return false;
    }
    let diff = if new >= old { new - old } else { old - new };
    let bps = diff.saturating_mul(10_000) / old.abs();
    bps > max_bps as i128
}
