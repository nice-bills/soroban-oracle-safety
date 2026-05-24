use soroban_sdk::{Env, Symbol};

pub use oracle_storage::{
    extend_instance, get_source, has_admin, require_admin, set_admin, set_source,
};

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
