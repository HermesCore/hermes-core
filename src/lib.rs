//! Hermes Core Library
//!
//! This crate defines the core architecture and traits for the Hermes MEV Swarm Intelligence System.

pub mod agents;
pub mod swarm;
pub mod arbitrage;
pub mod execution;
pub mod learning;
pub mod infra;

pub use agents::*;
pub use swarm::*;
pub use arbitrage::*;
pub use execution::*;
pub use learning::*;
pub use infra::*;

/// Core trait representing an autonomous Hermes agent.
pub trait HermesAgent {
    fn id(&self) -> String;
    fn tick(&mut self, timestamp: u64);
    fn observe(&mut self, observation: Observation);
    fn act(&mut self) -> Action;
}

/// Observation fed into an agent from the environment.
#[derive(Debug, Clone)]
pub struct Observation {
    pub market_data: MarketData,
    pub agent_state: AgentState,
}

/// An action an agent may emit, typically representing an intent to trade or signal.
#[derive(Debug, Clone)]
pub enum Action {
    PlaceOrder(Order),
    MultiOrder(Vec<Order>),
    BroadcastSignal(Signal),
    NoOp,
}

/// Placeholder market data structure.
#[derive(Debug, Clone)]
pub struct MarketData {
    pub price_map: std::collections::HashMap<String, f64>,
    pub block_info: BlockInfo,
}

/// Basic state of an agent (can be extended for learning).
#[derive(Debug, Clone)]
pub struct AgentState {
    pub memory_hash: String,
    pub last_action: Option<Action>,
    pub agent_health: f32,
}

/// Basic order structure for execution.
#[derive(Debug, Clone)]
pub struct Order {
    pub pair: String,
    pub amount: f64,
    pub side: OrderSide,
    pub dex: String,
    pub pool_address: String,
    pub fee: f64,
}

/// Buy/Sell enum.
#[derive(Debug, Clone)]
pub enum OrderSide {
    Buy,
    Sell,
}

/// Basic swarm-wide signal.
#[derive(Debug, Clone)]
pub struct Signal {
    pub origin_id: String,
    pub payload: String,
}

/// Block-level info for context awareness.
#[derive(Debug, Clone)]
pub struct BlockInfo {
    pub number: u64,
    pub timestamp: u64,
    pub gas_price: u64,
}
