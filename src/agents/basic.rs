use crate::agents::memory::AgentMemory;
use crate::agents::types::AgentRole;
use crate::arbitrage::MEVStrategy;
use crate::{Observation, Action, Order, OrderSide, MarketData, BlockInfo, AgentState, Signal};
use crate::execution::bundle::BundleExecutor;

use std::collections::HashSet;
use std::sync::Arc;

#[derive(Clone)]
pub struct BasicReactiveAgent {
    pub id: String,
    pub memory: AgentMemory,
    pub reputation_score: f64,
    pub threshold: f64,
    pub role: AgentRole,
    pub generation: u32,
    pub alerted_pairs: HashSet<String>,
    pub mentor_id: Option<String>,
    pub parent_id: Option<String>,
    pub strategy: Option<Arc<dyn MEVStrategy>>,
    pub current_mode: Option<String>,
    pub last_market_data: MarketData,
}

impl BasicReactiveAgent {
    pub fn new(id: &str, threshold: f64) -> Self {
        Self {
            id: id.to_string(),
            memory: AgentMemory::new(),
            reputation_score: 0.0,
            threshold,
            role: AgentRole::Explorer,
            generation: 0,
            alerted_pairs: HashSet::new(),
            mentor_id: None,
            parent_id: None,
            strategy: None,
            current_mode: None,
            last_market_data: MarketData {
                price_map: Default::default(),
                block_info: BlockInfo {
                    number: 0,
                    timestamp: 0,
                    gas_price: 0,
                },
            },
        }
    }

    pub fn set_strategy(&mut self, strategy: Arc<dyn MEVStrategy>) {
        self.strategy = Some(strategy);
    }

    pub fn peek_next_action(&self) -> Option<Action> {
        if self.alerted_pairs.contains("ETH/USDC") {
            let orders = self.build_orders_for("ETH/USDC");
            Some(Action::MultiOrder(orders))
        } else {
            None
        }
    }

    pub fn build_orders_for(&self, pair: &str) -> Vec<Order> {
        vec![
            Order {
                pair: pair.into(),
                amount: 1.0,
                side: OrderSide::Buy,
                dex: "uniswap".into(),
                pool_address: "0x0000000000000000000000000000000000000000".into(),
                fee: 0.003,
            },
            Order {
                pair: pair.into(),
                amount: 1.0,
                side: OrderSide::Sell,
                dex: "curve".into(),
                pool_address: "0x0000000000000000000000000000000000000000".into(),
                fee: 0.0005,
            },
        ]
    }
}

impl crate::agents::Agent for BasicReactiveAgent {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn tick(&mut self, _timestamp: u64) {
        // placeholder: score decay, mutation trigger, etc.
    }

    fn observe(&mut self, observation: Observation) {
        self.last_market_data = observation.market_data.clone();

        if let Some(Action::BroadcastSignal(sig)) = &observation.agent_state.last_action {
            if sig.payload.starts_with("goal:") {
                let parts: Vec<&str> = sig.payload.split(':').collect();
                if parts.len() == 3 {
                    let target = parts[2];
                    self.alerted_pairs.insert(target.to_string());
                }
            }
        }
    }

    fn act(&mut self) -> Action {
        if let Some(strategy) = &self.strategy {
            if let Some(opp) = strategy.detect_opportunity(&self.last_market_data) {
                return Action::MultiOrder(opp.orders);
            }
        }

        Action::NoOp
    }

    fn health(&self) -> f32 {
        1.0
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
