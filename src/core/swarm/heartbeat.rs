//! Heartbeat module
//! Emits and processes periodic health pings from agents to the swarm supervisor.

// --- Imports ---
// use crate::core::swarm::swarm_supervisor::AgentHeartbeat;

/// Represents a ping emitted by an active agent
pub fn create_heartbeat(agent_id: &str, timestamp: u64) -> crate::core::swarm::swarm_supervisor::AgentHeartbeat {
    crate::core::swarm::swarm_supervisor::AgentHeartbeat {
        agent_id: agent_id.to_string(),
        timestamp,
    }
}
