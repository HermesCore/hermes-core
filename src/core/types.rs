// core/types.rs

/// Shared signal format across swarm agents
#[derive(Debug, Clone)]
pub struct SwarmSignal {
    pub signal_id: String,
    pub signal_type: String,
    pub confidence: f64,
    pub timestamp: u64,
    pub ttl_seconds: u64,
}

/// Shared AgentId alias for swarm registry & routing
pub type AgentId = String;
