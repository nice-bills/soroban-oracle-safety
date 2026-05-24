use sep_40_oracle::{Asset, PriceData};
use soroban_sdk::{Env, Symbol};

pub use oracle_storage::{
    extend_instance, get_source, has_admin, require_admin, set_admin, set_source,
};

#[derive(Clone)]
#[soroban_sdk::contracttype]
pub struct CircuitBreakerConfig {
    /// Reject prices older than this many seconds vs ledger time.
    pub max_staleness_secs: u64,
    /// Max jump vs previous price in basis points (100 = 1%). Zero disables deviation checks only.
    pub max_deviation_bps: u32,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            max_staleness_secs: 300,
            max_deviation_bps: 500,
        }
    }
}

impl CircuitBreakerConfig {
    pub fn validate(&self) -> Result<(), oracle_storage::AdapterError> {
        if self.max_staleness_secs == 0 {
            return Err(oracle_storage::AdapterError::InvalidConfig);
        }
        Ok(())
    }
}

pub fn get_config(env: &Env) -> CircuitBreakerConfig {
    env.storage()
        .instance()
        .get(&Symbol::new(env, "Config"))
        .expect("not initialized")
}

pub fn set_config(env: &Env, config: &CircuitBreakerConfig) {
    env.storage()
        .instance()
        .set(&Symbol::new(env, "Config"), config);
}

pub fn get_last_price(env: &Env, asset: &Asset) -> Option<PriceData> {
    let key = (Symbol::new(env, "LastP"), asset.clone());
    env.storage().instance().get(&key)
}

pub fn set_last_price(env: &Env, asset: &Asset, data: &PriceData) {
    let key = (Symbol::new(env, "LastP"), asset.clone());
    env.storage().instance().set(&key, data);
}
