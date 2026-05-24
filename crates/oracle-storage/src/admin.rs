use soroban_sdk::{Address, Env};

use crate::error::AdapterError;
use crate::keys::admin_key;

pub fn has_admin(env: &Env) -> bool {
    env.storage().instance().has(&admin_key(env))
}

pub fn get_admin(env: &Env) -> Result<Address, AdapterError> {
    env.storage()
        .instance()
        .get(&admin_key(env))
        .ok_or(AdapterError::NotInitialized)
}

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&admin_key(env), admin);
}

pub fn require_admin(env: &Env, who: &Address) -> Result<(), AdapterError> {
    let admin = get_admin(env)?;
    if who != &admin {
        return Err(AdapterError::NotAdmin);
    }
    Ok(())
}
