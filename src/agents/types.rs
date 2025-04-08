//! Agent Types
//!
//! Defines different kinds of agents in the system.

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AgentRole {
    Explorer,
    Sniper,
    Surfer,
    Generalist,
}
