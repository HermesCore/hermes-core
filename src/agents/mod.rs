//! Agents Module
//!
//! This module defines agent behaviors, memory management, and agent types in the Hermes Swarm.

pub mod basic;
pub mod memory;
pub mod types;

use crate::{Observation, Action};
use std::any::Any;

/// Core trait all Hermes agents must implement.
pub trait Agent: Send + Sync {
    fn id(&self) -> String;
    fn tick(&mut self, timestamp: u64);
    fn observe(&mut self, observation: Observation);
    fn act(&mut self) -> Action;
    fn health(&self) -> f32;
    fn as_any(&self) -> &dyn Any;
}
