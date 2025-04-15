// core/reputation.rs

use std::collections::HashMap;

pub struct ReputationTracker {
    reputation: HashMap<String, f64>, // Key: Signal ID, Value: Reputation score
}

impl ReputationTracker {
    // Create a new instance of ReputationTracker
    pub fn new() -> Self {
        ReputationTracker {
            reputation: HashMap::new(),
        }
    }

    // Update reputation for a given signal
    pub fn update_reputation(&mut self, signal_id: &str, reputation_score: f64) {
        let entry = self.reputation.entry(signal_id.to_string()).or_insert(0.0);
        *entry += reputation_score; // Update reputation based on the signal's contribution
    }

    // Get the reputation score of a signal
    pub fn get_reputation(&self, signal_id: &str) -> Option<&f64> {
        self.reputation.get(signal_id)
    }

    // Get the most trusted signal
    pub fn highest_trusted_signal(&self) -> Option<(&String, &f64)> {
        self.reputation.iter().max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
    }
}
