
//! Arbitrage Opportunity Analysis
//!
//! Utilities for modeling, simulating, and scoring arbitrage paths.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TokenPair {
    pub base: String,
    pub quote: String,
}

#[derive(Debug, Clone)]
pub struct PriceEdge {
    pub pair: TokenPair,
    pub price: f64, // quote per base
    pub liquidity: f64,
    pub fee: f64,
    pub dex: String,
    pub pool_address: String,
}

#[derive(Debug, Clone)]
pub struct ArbitragePath {
    pub route: Vec<TokenPair>,
    pub total_gas: u64,
    pub expected_profit: f64,
}

impl ArbitragePath {
    pub fn score(&self) -> f64 {
        self.expected_profit - self.total_gas as f64 * 0.000_000_1 // crude gas cost multiplier
    }
}

/// Build all 2-hop cycles for a given token.
pub fn find_simple_cycles(prices: &[PriceEdge], start_token: &str) -> Vec<ArbitragePath> {
    let mut paths = Vec::new();

    for a in prices {
        if a.pair.base != start_token {
            continue;
        }

        for b in prices {
            if a.pair.quote == b.pair.base && b.pair.quote == start_token {
                let profit = simulate_cycle(&[a, b]);
                paths.push(ArbitragePath {
                    route: vec![a.pair.clone(), b.pair.clone()],
                    total_gas: 21000 * 2,
                    expected_profit: profit,
                });
            }
        }
    }

    paths
}

/// Simulate a simple cycle to estimate profit.
pub fn simulate_cycle(edges: &[&PriceEdge]) -> f64 {
    let mut value = 1.0;

    for edge in edges {
        let fee_factor = 1.0 - edge.fee;
        value = value * edge.price * fee_factor;
    }

    value - 1.0 // profit over initial value
}
