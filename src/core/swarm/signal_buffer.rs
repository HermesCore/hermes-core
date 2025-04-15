// core/swarm/signal_buffer.rs

use std::collections::HashMap;

/// A signal cached in memory with TTL metadata
#[derive(Debug, Clone)]
pub struct BufferedSignal {
    pub signal_id: String,
    pub timestamp: u64,
    pub ttl_seconds: u64,
}

impl BufferedSignal {
    pub fn is_expired(&self, now: u64) -> bool {
        let expired = now > self.timestamp + self.ttl_seconds;
        if expired {
            println!("Signal {} expired at time {} (TTL: {})", self.signal_id, now, self.ttl_seconds); // Debug statement
        }
        expired
    }
}

/// In-memory buffer of recently seen signals
#[derive(Debug)]
pub struct SignalBuffer {
    pub buffer: HashMap<String, BufferedSignal>,
}

impl SignalBuffer {
    /// Create a new empty buffer
    pub fn new() -> Self {
        Self {
            buffer: HashMap::new(),
        }
    }

    /// Insert a signal into the buffer
    pub fn insert(&mut self, signal: BufferedSignal) {
        self.buffer.insert(signal.signal_id.clone(), signal);
    }

    /// Check if a signal is a duplicate and still active (not expired)
    pub fn is_duplicate(&self, signal_id: &str, now: u64) -> bool {
        match self.buffer.get(signal_id) {
            Some(sig) if !sig.is_expired(now) => true,
            _ => false,
        }
    }

    /// Remove all expired signals from the buffer
    pub fn prune(&mut self, now: u64) {
        self.buffer.retain(|_, sig| !sig.is_expired(now));
    }

    /// Count how many signals are still active
    pub fn active_count(&self, now: u64) -> usize {
        let active_signals = self.buffer.values().filter(|sig| !sig.is_expired(now)).count();
        println!("Active signals count at time {}: {}", now, active_signals); // Debug print
        active_signals
    }
}
