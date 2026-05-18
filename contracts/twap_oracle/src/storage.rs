use soroban_sdk::{Address, Env, Symbol};

const INSTANCE_THRESHOLD: u32 = 17_280;
const INSTANCE_BUMP: u32 = 34_560;

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

pub fn get_periods(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&Symbol::new(env, "Periods"))
        .expect("not initialized")
}

pub fn set_periods(env: &Env, periods: u32) {
    env.storage()
        .instance()
        .set(&Symbol::new(env, "Periods"), &periods);
}
