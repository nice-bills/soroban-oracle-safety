//! Off-chain configuration for oracle safety policies (used by TS examples and docs).

use serde::{Deserialize, Serialize};

/// Policy for circuit-breaker wrapper (seconds and basis points).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CircuitBreakerConfig {
    /// Reject prices older than this many seconds vs ledger time.
    pub max_staleness_secs: u64,
    /// Max allowed jump vs previous price, in basis points (100 = 1%).
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

/// TWAP window configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TwapConfig {
    /// Number of oracle resolution periods to average.
    pub periods: u32,
}

impl Default for TwapConfig {
    fn default() -> Self {
        Self { periods: 5 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_roundtrip_json() {
        let cfg = CircuitBreakerConfig::default();
        let json = serde_json::to_string(&cfg).unwrap();
        let back: CircuitBreakerConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(cfg, back);
    }
}
