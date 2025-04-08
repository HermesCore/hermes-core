//! Arbitrage Module
//!
//! Contains traits and structures related to MEV opportunity detection and strategy execution.

pub mod strategies;
pub mod opportunity;

use crate::MarketData;

/// Trait for MEV strategies (pluggable into agents or swarm).
pub trait MEVStrategy: Send + Sync {
    /// Detect if an opportunity exists in the given market data.
    fn detect_opportunity(&self, market_data: &MarketData) -> Option<ArbOpportunity>;

    /// Return a score indicating priority or profitability (used for sorting).
    fn score(&self, opportunity: &ArbOpportunity) -> f64;
}

/// A simplified opportunity struct (to be extended).
#[derive(Debug, Clone)]
pub struct ArbOpportunity {
    pub description: String,
    pub profit_estimate: f64,
    pub involved_tokens: Vec<String>,
    pub orders: Vec<crate::Order>,
}
