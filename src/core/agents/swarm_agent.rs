use std::collections::HashMap;
use rand::Rng;
use rand::seq::SliceRandom;
use crate::core::swarm::swarm_state::SwarmState;

#[derive(Debug, Clone, PartialEq)]
pub enum AgentClass {
    Scout,
    Sniper,
    Specialist,
    Executor,
    Sentinel,
}

#[derive(Debug, Clone)]
pub struct Agent {
    pub agent_id: String,
    pub class: AgentClass,
    pub score: f64,
    pub vault_balance: f64,
    pub confidence: f64,
    pub last_executed: u64,
    pub cooldown: u64,
    pub parent_id: Option<String>,
    pub generation: u32,
}

impl Agent {
    pub fn new(agent_id: &str, class: AgentClass) -> Self {
        let (score, vault_balance, cooldown) = match class {
            AgentClass::Scout => (2.0, 10.0, 5),
            AgentClass::Sniper => (5.0, 50.0, 15),
            AgentClass::Specialist => (8.0, 40.0, 30),
            AgentClass::Executor => (4.0, 100.0, 10),
            AgentClass::Sentinel => (6.0, 5.0, 60),
        };

        Self {
            agent_id: agent_id.to_string(),
            class,
            score,
            vault_balance,
            confidence: 1.0,
            last_executed: 0,
            cooldown,
            parent_id: None,
            generation: 1,
        }
    }

    pub fn execute(&mut self, signal_type: &str, now: u64) -> f64 {
        if now < self.last_executed + self.cooldown {
            return 0.0;
        }
        self.last_executed = now;
        let mut rng = rand::thread_rng();
        let profit = if rng.gen_bool(0.8) {
            rng.gen_range(0.5..5.0)
        } else {
            -rng.gen_range(0.1..1.0)
        };
        self.vault_balance += profit;
        self.score += profit;
        profit
    }

    pub fn update_confidence(&mut self, profit: f64) {
        self.confidence += profit * 0.1;
        if self.confidence < 0.1 {
            self.confidence = 0.1;
        }
    }

    pub fn evolve(&mut self) {
        if self.score > 20.0 && self.class == AgentClass::Sniper {
            self.class = AgentClass::Specialist;
            self.cooldown = 30;
        } else if self.score < 0.0 && self.class == AgentClass::Specialist {
            self.class = AgentClass::Sentinel;
            self.cooldown = 60;
        }
    }
}

pub struct SwarmAgentRegistry {
    agents: HashMap<String, Agent>,
}

impl SwarmAgentRegistry {
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
        }
    }

    pub fn add_agent(&mut self, agent: Agent) {
        self.agents.insert(agent.agent_id.clone(), agent);
    }

    pub fn get_all_agents(&self) -> Vec<&Agent> {
        self.agents.values().collect()
    }

    pub fn export_scores(&self) -> HashMap<String, f64> {
        self.agents.iter().map(|(id, agent)| (id.clone(), agent.score)).collect()
    }

    pub fn execute_agent(&mut self, agent_id: &str, signal_type: &str, now: u64) -> f64 {
        if let Some(agent) = self.agents.get_mut(agent_id) {
            let profit = agent.execute(signal_type, now);
            agent.update_confidence(profit);
            agent.evolve();
            return profit;
        }
        0.0
    }

    pub fn from_state(state: &SwarmState) -> Self {
        let mut registry = SwarmAgentRegistry::new();
        for agent in state.agents.values() {
            registry.add_agent(agent.clone());
        }
        registry
    }
}

pub struct HermesAgentFactory;

impl HermesAgentFactory {
    pub fn create_random() -> Agent {
        let classes = vec![
            AgentClass::Scout,
            AgentClass::Sniper,
            AgentClass::Specialist,
            AgentClass::Executor,
            AgentClass::Sentinel,
        ];
        let mut rng = rand::thread_rng();
        let class = classes.choose(&mut rng).unwrap().clone();
        let id = format!("agent{}", rng.gen_range(1000..9999));
        Agent::new(&id, class)
    }
}