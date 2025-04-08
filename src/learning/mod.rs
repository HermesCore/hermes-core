//! Learning Module
//!
//! Provides traits and utilities for adaptive agent behavior and evolution.

pub mod reinforcement;
pub mod evolution;

use crate::{Observation, Action};

/// Core trait for any learning-enabled component.
pub trait Learner: Send + Sync {
    fn update(&mut self, observation: &Observation, action: &Action, reward: f64);
    fn should_mutate(&self) -> bool;
    fn mutate(&self) -> Box<dyn Learner>;
}
