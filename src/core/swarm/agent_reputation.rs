// core/swarm/agent_reputation.rs

use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct AgentReputation {
    pub scores: HashMap<String, f64>,
}

impl AgentReputation {
    pub fn new() -> Self {
        Self {
            scores: HashMap::new(),
        }
    }

    /// Update agent reputation based on outcome
    pub fn apply_profit_feedback(&mut self, agent_id: &str, profit: f64, confidence: f64) {
        let entry = self.scores.entry(agent_id.to_string()).or_insert(0.0);
        let reward = profit * confidence * 0.5; // simple reward model
        *entry += reward;

        println!(
            "📈 Reputation Update: {} -> {:.2} (Δ {:.2})",
            agent_id, *entry, reward
        );
    }

    /// Apply decay to all reputation scores to prioritize recent activity
    pub fn decay_scores(&mut self, rate: f64) {
        for (agent_id, score) in self.scores.iter_mut() {
            let old = *score;
            *score *= 1.0 - rate;
            println!(
                "🕓 Reputation Decay: {} {:.2} → {:.2}",
                agent_id, old, *score
            );
        }
    }

    /// Print agent reputation state
    pub fn print_scores(&self) {
        println!("\n🌟 Agent Reputation Scores:");
        for (agent_id, score) in &self.scores {
            println!("  - {} → {:.2}", agent_id, score);
        }
    }
}
