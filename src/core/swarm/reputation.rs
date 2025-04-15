//! Reputation module
//! Tracks long-term agent trust levels across different signal types.

// --- Imports ---
// use crate::...;
// use std::collections::HashMap;

use std::collections::HashMap;

/// Reputation tracking for a specific agent across signal types.
pub struct AgentReputation {
    pub agent_id: String,
    pub trust_scores: HashMap<String, f64>, // e.g., "liquidation", "oracle-arb", etc.
}

impl AgentReputation {
    pub fn new(agent_id: &str) -> Self {
        Self {
            agent_id: agent_id.to_string(),
            trust_scores: HashMap::new(),
        }
    }

    /// Adjust trust score for a specific signal type
    pub fn adjust_score(&mut self, signal_type: &str, delta: f64) {
        let score = self.trust_scores.entry(signal_type.to_string()).or_insert(0.0);
        *score += delta;
    }

    /// Retrieve trust score for a specific signal type
    pub fn get_score(&self, signal_type: &str) -> f64 {
        *self.trust_scores.get(signal_type).unwrap_or(&0.0)
    }
}
