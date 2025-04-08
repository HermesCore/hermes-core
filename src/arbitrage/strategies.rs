//! MEV Strategies
//!
//! Contains basic strategy implementations.

use crate::arbitrage::{MEVStrategy, ArbOpportunity};
use crate::MarketData;
use super::opportunity::{PriceEdge, TokenPair, find_simple_cycles};

pub struct MultiDexArb;

impl MultiDexArb {
    pub fn new() -> Self {
        Self
    }
}

impl MEVStrategy for MultiDexArb {
    fn detect_opportunity(&self, market_data: &MarketData) -> Option<ArbOpportunity> {
        let prices = extract_price_edges(&market_data.price_map);
        let mut best: Option<ArbOpportunity> = None;

        for token in ["ETH", "USDC", "DAI", "WBTC"] {
            let paths = find_simple_cycles(&prices, token);
            for path in paths {
                if path.expected_profit > 0.0 {
                    let orders: Vec<crate::Order> = path.route.iter().enumerate().map(|(i, pair)| {
                        let edge = prices.iter().find(|e| e.pair == *pair).unwrap();
                        crate::Order {
                            pair: format!("{}/{}", pair.base, pair.quote),
                            amount: 1.0,
                            side: if i == 0 { crate::OrderSide::Buy } else { crate::OrderSide::Sell },
                            dex: edge.dex.clone(),
                            pool_address: edge.pool_address.clone(),
                            fee: edge.fee,
                        }
                    }).collect();

                    best = Some(ArbOpportunity {
                        description: format!("2-hop arb: {:?}", path.route),
                        profit_estimate: path.score(),
                        involved_tokens: path.route.iter()
                            .flat_map(|p| vec![p.base.clone(), p.quote.clone()])
                            .collect(),
                        orders,
                    });
                }
            }
        }

        best
    }

    fn score(&self, opportunity: &ArbOpportunity) -> f64 {
        opportunity.profit_estimate
    }
}

/// Converts a raw price map into edge format for graph simulation.
fn extract_price_edges(price_map: &std::collections::HashMap<String, f64>) -> Vec<PriceEdge> {
    let mut edges = Vec::new();

    for (pair_str, price) in price_map {
        if let Some((dex, pair, pool, fee)) = parse_key(pair_str) {
            let tokens: Vec<&str> = pair.split('/').collect();
            if tokens.len() == 2 {
                edges.push(PriceEdge {
                    pair: TokenPair {
                        base: tokens[0].to_string(),
                        quote: tokens[1].to_string(),
                    },
                    price: *price,
                    liquidity: 1_000_000.0,
                    fee,
                    dex,
                    pool_address: pool,
                });
            }
        }
    }

    edges
}

fn parse_key(key: &str) -> Option<(String, String, String, f64)> {
    let parts: Vec<&str> = key.split(':').collect();
    if parts.len() == 4 {
        let dex = parts[0].to_string();
        let pair = parts[1].to_string();
        let pool = parts[2].to_string();
        let fee = parts[3].parse().ok()?;
        Some((dex, pair, pool, fee))
    } else {
        None
    }
}
