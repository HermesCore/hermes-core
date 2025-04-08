//! Evolutionary Learning
//!
//! Defines mutation, crossover, and fitness evaluation for evolving agents.

use rand::Rng;

#[derive(Debug, Clone)]
pub struct Genome {
    pub aggressiveness: f64,
    pub risk_tolerance: f64,
    pub memory_span: u32,
    pub strategy_id: String,
}

impl Genome {
    /// Create a randomized genome.
    pub fn random(strategy_id: &str) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            aggressiveness: rng.gen_range(0.0..1.0),
            risk_tolerance: rng.gen_range(0.0..1.0),
            memory_span: rng.gen_range(10..100),
            strategy_id: strategy_id.into(),
        }
    }

    /// Mutate the genome slightly.
    pub fn mutate(&self) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            aggressiveness: (self.aggressiveness + rng.gen_range(-0.1..0.1)).clamp(0.0, 1.0),
            risk_tolerance: (self.risk_tolerance + rng.gen_range(-0.1..0.1)).clamp(0.0, 1.0),
            memory_span: (self.memory_span as i32 + rng.gen_range(-5..5)).clamp(5, 200) as u32,
            strategy_id: self.strategy_id.clone(),
        }
    }

    /// Combine traits from two parents.
    pub fn crossover(a: &Genome, b: &Genome) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            aggressiveness: if rng.gen_bool(0.5) { a.aggressiveness } else { b.aggressiveness },
            risk_tolerance: if rng.gen_bool(0.5) { a.risk_tolerance } else { b.risk_tolerance },
            memory_span: if rng.gen_bool(0.5) { a.memory_span } else { b.memory_span },
            strategy_id: format!("{}_x_{}", a.strategy_id, b.strategy_id),
        }
    }

    /// Dummy fitness score (to be replaced with real sim results).
    pub fn fitness(&self) -> f64 {
        self.aggressiveness * 0.5 + self.risk_tolerance * 0.5
    }
}
