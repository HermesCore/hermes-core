//! Reinforcement Learning
//!
//! A simple Q-learning based agent memory.

use crate::learning::Learner;
use crate::{Observation, Action};
use std::collections::HashMap;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct QLearner {
    pub q_table: HashMap<(String, String), f64>, // (state, action) -> value
    pub learning_rate: f64,
    pub discount: f64,
    pub epsilon: f64, // for exploration
}

impl QLearner {
    pub fn new() -> Self {
        Self {
            q_table: HashMap::new(),
            learning_rate: 0.1,
            discount: 0.9,
            epsilon: 0.1,
        }
    }

    fn encode_state(observation: &Observation) -> String {
        format!("{:?}", observation.market_data.price_map)
    }

    fn encode_action(action: &Action) -> String {
        format!("{:?}", action)
    }
}

impl Learner for QLearner {
    fn update(&mut self, observation: &Observation, action: &Action, reward: f64) {
        let state = Self::encode_state(observation);
        let act = Self::encode_action(action);
        let key = (state.clone(), act.clone());

        let old_value = *self.q_table.get(&key).unwrap_or(&0.0);

        let max_future = self.q_table
            .iter()
            .filter(|((s, _), _)| s == &state)
            .map(|(_, v)| *v)
            .fold(0.0, f64::max);

        let new_value = old_value + self.learning_rate * (reward + self.discount * max_future - old_value);
        self.q_table.insert(key, new_value);
    }

    fn should_mutate(&self) -> bool {
        rand::thread_rng().gen_bool(self.epsilon.min(1.0).max(0.0))
    }

    fn mutate(&self) -> Box<dyn Learner> {
        let mut rng = rand::thread_rng();
        let mut new = self.clone();
        new.learning_rate += rng.gen_range(-0.01..0.01);
        new.discount += rng.gen_range(-0.05..0.05);
        new.epsilon += rng.gen_range(-0.02..0.02);
        Box::new(new)
    }
}
