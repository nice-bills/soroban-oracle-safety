use soroban_sdk::{Env, Symbol};

pub fn admin_key(env: &Env) -> Symbol {
    Symbol::new(env, "Admin")
}

pub fn source_key(env: &Env) -> Symbol {
    Symbol::new(env, "Source")
}
