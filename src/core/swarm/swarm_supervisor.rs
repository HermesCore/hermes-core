use std::collections::HashMap;
use crate::core::types::AgentId;
use crate::core::vault_growth_scaler::VaultSnapshot;
use crate::core::agents::swarm_agent::Agent;

pub struct AgentHeartbeat {
    pub agent_id: String,
    pub timestamp: u64,
}

pub struct SwarmState {
    pub agents: HashMap<AgentId, Agent>,
}

impl SwarmState {
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
        }
    }

    pub fn add_agent(&mut self, agent: Agent) {
        self.agents.insert(agent.agent_id.clone(), agent);
    }

    pub fn remove_agent(&mut self, agent_id: &AgentId) {
        self.agents.remove(agent_id);
    }

    pub fn agent_count(&self) -> usize {
        self.agents.len()
    }

    pub fn get_lowest_performers(
        &self,
        count: usize,
        snapshot: &VaultSnapshot,
    ) -> Vec<AgentId> {
        let mut sorted: Vec<_> = snapshot
            .balances.iter()
            .filter(|(id, _)| self.agents.contains_key(*id))
            .collect();

        sorted.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());

        sorted
            .iter()
            .take(count)
            .map(|(id, _)| (*id).clone())
            .collect()
    }
}