// core/swarm/mod.rs

pub mod agent_scores;
pub mod reputation;
pub mod score_decay;
pub mod score_boost;
pub mod swarm_supervisor;
pub mod swarm_state;
pub mod swarm_scaler;
pub mod signal_buffer;
pub mod cooldown_manager;
pub mod heartbeat;
pub mod swarm_logger;


// ✅ Added this line to register the self-learning reputation module
#[path = "agent_reputation.rs"]
pub mod agent_reputation;
