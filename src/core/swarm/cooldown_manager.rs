//! Cooldown Manager module
//! Enforces time-based cooldowns per agent or signal type to prevent over-execution.

// --- Imports ---
// use std::collections::HashMap;

use std::collections::HashMap;

/// Tracks the last execution time of actions per agent or signal type
pub struct CooldownManager {
    pub cooldown_period_secs: u64,
    pub last_action_times: HashMap<String, u64>, // key: agent_id or signal_type
}

impl CooldownManager {
    pub fn new(cooldown_secs: u64) -> Self {
        Self {
            cooldown_period_secs: cooldown_secs,
            last_action_times: HashMap::new(),
        }
    }

    /// Checks whether an agent is in cooldown
    pub fn is_on_cooldown(&self, key: &str, current_time: u64) -> bool {
        match self.last_action_times.get(key) {
            Some(&last) => current_time < last + self.cooldown_period_secs,
            None => false,
        }
    }

    /// Updates the last action timestamp
    pub fn update_timestamp(&mut self, key: &str, current_time: u64) {
        self.last_action_times.insert(key.to_string(), current_time);
    }
}
