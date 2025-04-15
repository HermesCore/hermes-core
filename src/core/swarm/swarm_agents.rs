use std::collections::HashMap;
use rand::Rng;

/// Defines the role or specialization of an agent
#[derive(Debug, Clone, PartialEq)]
pub enum AgentClass {
    Scout,
    Sniper,
    Specialist,
    Executor,
    Sentinel,
}

/// Represents a single agent in the swarm
#[derive(Debug, Clone)]
pub struct Agent {
    pub agent_id: String,
    pub class: AgentClass,
    pub score: f64,
    pub vault_balance: f64,
    pub confidence: f64,
    pub last_executed: u64,
    pub cooldown: u64,
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
        }
    }

    pub fn execute(&mut self, signal_type: &str, now: u64) -> f64 {
        if now < self.last_executed + self.cooldown {
            println!(
                "[{}] Cooldown in effect, cannot execute {} signal.",
                self.agent_id, signal_type
            );
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

        println!(
            "[{}] Executed {} signal, profit: {:.4} ETH, new balance: {:.4} ETH",
            self.agent_id, signal_type, profit, self.vault_balance
        );

        profit
    }

    pub fn update_confidence(&mut self, profit: f64) {
        self.confidence += profit * 0.1;
        if self.confidence < 0.1 {
            self.confidence = 0.1;
        }
        println!(
            "[{}] Confidence updated: {:.2}",
            self.agent_id, self.confidence
        );
    }

    pub fn evolve(&mut self) {
        if self.score > 20.0 && self.class == AgentClass::Sniper {
            self.class = AgentClass::Specialist;
            self.cooldown = 30;
            println!("[{}] Promoted to Specialist", self.agent_id);
        } else if self.score < 0.0 && self.class == AgentClass::Specialist {
            self.class = AgentClass::Sentinel;
            self.cooldown = 60;
            println!("[{}] Demoted to Sentinel", self.agent_id);
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

    pub fn get_agent(&self, agent_id: &str) -> Option<&Agent> {
        self.agents.get(agent_id)
    }

    pub fn get_all_agents(&self) -> Vec<&Agent> {
        self.agents.values().collect()
    }
    pub fn export_scores(&self) -> HashMap<String, f64> {
        self.agents.iter().map(|(id, agent)| (id.clone(), agent.score)).collect()
    }


    /// ✅ Now returns the agent’s profit for reputation tracking
    pub fn execute_agent(&mut self, agent_id: &str, signal_type: &str, now: u64) -> f64 {
        if let Some(agent) = self.agents.get_mut(agent_id) {
            let profit = agent.execute(signal_type, now);
            agent.update_confidence(profit);
            agent.evolve();
            return profit;
        }
        0.0
    }

    pub fn update_agent_score(&mut self, agent_id: &str, score_delta: f64) {
        if let Some(agent) = self.agents.get_mut(agent_id) {
            agent.score += score_delta;
        }
    }
}
