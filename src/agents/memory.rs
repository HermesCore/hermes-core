//! Agent Memory
//!
//! Memory structures for learning, pattern recall, and historical context.

use crate::Action;

#[derive(Debug, Clone)]
pub struct AgentMemory {
    pub recent_actions: Vec<Action>,
    pub memory_hash: String,
    pub score: f64,
}

impl AgentMemory {
    pub fn new() -> Self {
        Self {
            recent_actions: Vec::new(),
            memory_hash: String::new(),
            score: 0.0,
        }
    }

    pub fn record_action(&mut self, action: &Action) {
        self.recent_actions.push(action.clone());
        if self.recent_actions.len() > 50 {
            self.recent_actions.remove(0);
        }
    }

    pub fn summarize(&self) -> String {
        format!("Score: {:.2}, MemoryHash: {}", self.score, self.memory_hash)
    }
}
