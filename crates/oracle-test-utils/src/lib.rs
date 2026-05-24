//! Shared deploy helpers for contract unit and integration tests.

use circuit_breaker::{CircuitBreaker, CircuitBreakerClient};

pub use circuit_breaker::CircuitBreakerConfig;
use mock_feed::{MockFeed, MockFeedClient};
use sep_40_oracle::Asset;
use soroban_sdk::{Address, Env};
use twap_oracle::{TwapOracle, TwapOracleClient};

pub fn deploy_mock<'a>(env: &'a Env, admin: &Address) -> (Address, MockFeedClient<'a>) {
    let base = Asset::Other(soroban_sdk::symbol_short!("USD"));
    let id = env.register(MockFeed, ());
    let client = MockFeedClient::new(env, &id);
    client.initialize(admin, &base, &7, &60);
    (id, client)
}

pub fn deploy_twap<'a>(
    env: &'a Env,
    admin: &Address,
    source: &Address,
    periods: u32,
) -> TwapOracleClient<'a> {
    let id = env.register(TwapOracle, ());
    let client = TwapOracleClient::new(env, &id);
    client.initialize(admin, source, &periods);
    client
}

pub fn deploy_breaker<'a>(
    env: &'a Env,
    admin: &Address,
    source: &Address,
    config: CircuitBreakerConfig,
) -> CircuitBreakerClient<'a> {
    let id = env.register(CircuitBreaker, ());
    let client = CircuitBreakerClient::new(env, &id);
    client.initialize(admin, source, &config);
    client
}

/// mock → twap → breaker (recommended production chain).
pub struct AdapterChain<'a> {
    pub mock_id: Address,
    pub mock: MockFeedClient<'a>,
    pub twap: TwapOracleClient<'a>,
    pub breaker: CircuitBreakerClient<'a>,
}

pub fn deploy_chain<'a>(
    env: &'a Env,
    admin: &Address,
    periods: u32,
    breaker_config: CircuitBreakerConfig,
) -> AdapterChain<'a> {
    let (mock_id, mock) = deploy_mock(env, admin);
    let twap_id = env.register(TwapOracle, ());
    let twap = TwapOracleClient::new(env, &twap_id);
    twap.initialize(admin, &mock_id, &periods);
    let breaker = deploy_breaker(env, admin, &twap_id, breaker_config);
    AdapterChain {
        mock_id,
        mock,
        twap,
        breaker,
    }
}
