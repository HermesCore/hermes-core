// core/agents/sniper_agent.rs

use crate::core::swarm::signal_buffer::{BufferedSignal, SignalBuffer};

pub struct SniperAgent {
    id: String,
    score: f64,
    last_action_time: u64,
    buffer: SignalBuffer,
}

impl SniperAgent {
    pub fn new(id: String) -> Self {
        Self {
            id,
            score: 50.0, // Starting score
            last_action_time: 0,
            buffer: SignalBuffer::new(),
        }
    }

    pub fn perform_action(&mut self, current_time: u64, signal_id: String) {
        // Insert the signal into the buffer
        let signal = BufferedSignal {
            signal_id: signal_id.clone(),
            timestamp: current_time,
            ttl_seconds: 20,
        };
        self.buffer.insert(signal);

        // Check if the signal is a duplicate
        if self.buffer.is_duplicate(&signal_id, current_time) {
            println!("[{}] Signal {} is a duplicate, ignoring.", self.id, signal_id);
            return;
        }

        // Simulate action execution (e.g., profit calculation)
        let profit = 10.0; // Simulated profit
        self.score += profit; // Update score
        println!("[{}] Executed action on signal {}. Profit: {:.2}", self.id, signal_id, profit);
    }
}

