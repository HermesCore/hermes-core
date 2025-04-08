//! Swarm Module
//!
//! Manages agent-to-agent communication, signal broadcasting, and swarm coordination.

use crate::{Signal, Agent, Observation, AgentState, MarketData, Action};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Default)]
pub struct Swarm {
    pub agents: HashMap<String, Arc<RwLock<dyn Agent>>>,
}

impl Swarm {
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
        }
    }

    /// Add an agent to the swarm.
    pub fn register_agent(&mut self, agent: Arc<RwLock<dyn Agent>>) {
        let id = agent.read().unwrap().id();
        self.agents.insert(id, agent);
    }

    /// Broadcast a signal to all agents.
    pub fn broadcast(&self, signal: Signal) {
        for (_id, agent) in self.agents.iter() {
            let obs = Observation {
                market_data: MarketData {
                    price_map: Default::default(),
                    block_info: crate::BlockInfo {
                        number: 0,
                        timestamp: 0,
                        gas_price: 0,
                    },
                },
                agent_state: AgentState {
                    memory_hash: "".into(),
                    last_action: Some(Action::BroadcastSignal(signal.clone())),
                    agent_health: 1.0,
                },
            };
            agent.write().unwrap().observe(obs);
        }
    }

    /// Tick all agents with the current timestamp.
    pub fn tick_all(&self, timestamp: u64) {
        for agent in self.agents.values() {
            agent.write().unwrap().tick(timestamp);
        }
    }

    /// Collect all actions from agents in the swarm.
    pub fn collect_actions(&self) -> Vec<(String, Action)> {
        self.agents
            .iter()
            .map(|(id, agent)| (id.clone(), agent.read().unwrap().act()))
            .collect()
    }
}
