use sep_40_oracle::{Asset, PriceData};
use soroban_sdk::{Address, Env, Map, Symbol};

const INSTANCE_THRESHOLD: u32 = 17_280;
const INSTANCE_BUMP: u32 = 34_560;

#[derive(Clone)]
#[soroban_sdk::contracttype]
pub struct CircuitBreakerConfig {
    pub max_staleness_secs: u64,
    pub max_deviation_bps: u32,
}

pub fn extend_instance(env: &Env) {
    env.storage()
        .instance()
        .extend_ttl(INSTANCE_THRESHOLD, INSTANCE_BUMP);
}

pub fn has_admin(env: &Env) -> bool {
    env.storage().instance().has(&Symbol::new(env, "Admin"))
}

pub fn get_admin(env: &Env) -> Address {
    env.storage()
        .instance()
        .get(&Symbol::new(env, "Admin"))
        .expect("not initialized")
}

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage()
        .instance()
        .set(&Symbol::new(env, "Admin"), admin);
}

pub fn get_source(env: &Env) -> Address {
    env.storage()
        .instance()
        .get(&Symbol::new(env, "Source"))
        .expect("not initialized")
}

pub fn set_source(env: &Env, source: &Address) {
    env.storage()
        .instance()
        .set(&Symbol::new(env, "Source"), source);
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
    let map: Map<Asset, PriceData> = env
        .storage()
        .instance()
        .get(&Symbol::new(env, "LastPrice"))
        .unwrap_or_else(|| Map::new(env));
    map.get(asset.clone())
}

pub fn set_last_price(env: &Env, asset: &Asset, data: &PriceData) {
    let mut map: Map<Asset, PriceData> = env
        .storage()
        .instance()
        .get(&Symbol::new(env, "LastPrice"))
        .unwrap_or_else(|| Map::new(env));
    map.set(asset.clone(), data.clone());
    env.storage()
        .instance()
        .set(&Symbol::new(env, "LastPrice"), &map);
}
