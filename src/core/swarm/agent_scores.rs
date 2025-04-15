//! Agent Score module
//! Tracks adaptive performance scores for agents across multiple signal types.

use std::collections::HashMap;

/// Score value + last update time
#[derive(Debug, Clone)]
pub struct AgentScore {
    pub value: f64,
    pub last_updated: u64,
}

impl AgentScore {
    pub fn new() -> Self {
        Self {
            value: 0.0,
            last_updated: 0,
        }
    }

    pub fn update(&mut self, delta: f64, timestamp: u64) {
        self.value += delta;
        self.last_updated = timestamp;
    }

    /// ✅ This is the method that was missing
    pub fn apply_decay(&mut self, decay_factor: f64) {
        self.value *= decay_factor;
    }
}

/// agent_id -> signal_type -> score
#[derive(Debug, Default)]
pub struct AgentScoreMap {
    pub scores: HashMap<String, HashMap<String, AgentScore>>,
}

impl AgentScoreMap {
    pub fn new() -> Self {
        Self {
            scores: HashMap::new(),
        }
    }

    pub fn update_score(
        &mut self,
        agent_id: &str,
        signal_type: &str,
        delta: f64,
        timestamp: u64,
    ) {
        let agent_scores = self
            .scores
            .entry(agent_id.to_string())
            .or_insert_with(HashMap::new);

        let score = agent_scores
            .entry(signal_type.to_string())
            .or_insert_with(AgentScore::new);

        score.update(delta, timestamp);
    }

    pub fn apply_decay(&mut self, decay_factor: f64) {
        for agent_scores in self.scores.values_mut() {
            for score in agent_scores.values_mut() {
                score.apply_decay(decay_factor);
            }
        }
    }

    pub fn get_score(&self, agent_id: &str, signal_type: &str) -> f64 {
        self.scores
            .get(agent_id)
            .and_then(|m| m.get(signal_type))
            .map(|s| s.value)
            .unwrap_or(0.0)
    }
}
