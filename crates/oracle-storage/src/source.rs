use soroban_sdk::{Address, Env};

use crate::error::AdapterError;
use crate::keys::source_key;

pub fn get_source(env: &Env) -> Result<Address, AdapterError> {
    env.storage()
        .instance()
        .get(&source_key(env))
        .ok_or(AdapterError::NotInitialized)
}

pub fn set_source(env: &Env, source: &Address) {
    env.storage().instance().set(&source_key(env), source);
}
