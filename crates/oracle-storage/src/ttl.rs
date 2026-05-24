use soroban_sdk::Env;

const INSTANCE_THRESHOLD: u32 = 17_280;
const INSTANCE_BUMP: u32 = 34_560;

pub fn extend_instance(env: &Env) {
    env.storage()
        .instance()
        .extend_ttl(INSTANCE_THRESHOLD, INSTANCE_BUMP);
}
