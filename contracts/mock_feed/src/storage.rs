use sep_40_oracle::Asset;
use soroban_sdk::{Address, Env, Symbol, Vec};

const INSTANCE_THRESHOLD: u32 = 17_280;
const INSTANCE_BUMP: u32 = 34_560;
const TEMP_THRESHOLD: u32 = 17_280;
const TEMP_BUMP: u32 = 34_560;

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

pub fn get_base(env: &Env) -> Asset {
    env.storage()
        .instance()
        .get(&Symbol::new(env, "Base"))
        .expect("not initialized")
}

pub fn set_base(env: &Env, base: &Asset) {
    env.storage()
        .instance()
        .set(&Symbol::new(env, "Base"), base);
}

pub fn get_decimals(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&Symbol::new(env, "Decimals"))
        .expect("not initialized")
}

pub fn set_decimals(env: &Env, decimals: u32) {
    env.storage()
        .instance()
        .set(&Symbol::new(env, "Decimals"), &decimals);
}

pub fn get_resolution(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&Symbol::new(env, "Resolution"))
        .expect("not initialized")
}

pub fn set_resolution(env: &Env, resolution: u32) {
    env.storage()
        .instance()
        .set(&Symbol::new(env, "Resolution"), &resolution);
}

pub fn get_assets(env: &Env) -> Vec<Asset> {
    env.storage()
        .instance()
        .get(&Symbol::new(env, "Assets"))
        .unwrap_or_else(|| Vec::new(env))
}

pub fn set_assets(env: &Env, assets: &Vec<Asset>) {
    env.storage()
        .instance()
        .set(&Symbol::new(env, "Assets"), assets);
}

pub fn get_last_timestamp(env: &Env, asset: &Asset) -> Option<u64> {
    let key = (Symbol::new(env, "LastTs"), asset.clone());
    env.storage().instance().get(&key)
}

pub fn set_last_timestamp(env: &Env, asset: &Asset, timestamp: u64) {
    let key = (Symbol::new(env, "LastTs"), asset.clone());
    env.storage().instance().set(&key, &timestamp);
}

pub fn get_price(env: &Env, asset: &Asset, timestamp: u64) -> Option<i128> {
    let key = (asset.clone(), timestamp);
    env.storage().temporary().get(&key)
}

pub fn set_price(env: &Env, asset: &Asset, timestamp: u64, price: i128) {
    let key = (asset.clone(), timestamp);
    env.storage().temporary().set(&key, &price);
    env.storage()
        .temporary()
        .extend_ttl(&key, TEMP_THRESHOLD, TEMP_BUMP);
}
