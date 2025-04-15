// core/agent_scores.rs

use std::collections::HashMap;

pub struct AgentScores {
    scores: HashMap<String, f64>,
}

impl AgentScores {
    // Create a new instance of AgentScores
    pub fn new() -> Self {
        AgentScores {
            scores: HashMap::new(),
        }
    }

    // Update or add a new score for an agent
    pub fn update_score(&mut self, agent_id: &str, score: f64) {
        let entry = self.scores.entry(agent_id.to_string()).or_insert(0.0);
        *entry += score; // Accumulate score for the agent
    }

    // Get the score of an agent
    pub fn get_score(&self, agent_id: &str) -> Option<&f64> {
        self.scores.get(agent_id)
    }

    // Get the highest scoring agent
    pub fn highest_score_agent(&self) -> Option<(&String, &f64)> {
        self.scores.iter().max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
    }
}
