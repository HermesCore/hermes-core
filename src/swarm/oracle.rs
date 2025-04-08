//! Swarm Oracle
//!
//! Global observer of the swarm. Can trigger swarm-wide signals based on behavior and health.

use crate::{Swarm, Signal, Agent};
use std::collections::HashMap;

pub struct SwarmOracle;

impl SwarmOracle {
    /// Analyze the current swarm state and return global signals to broadcast.
    pub fn analyze(swarm: &Swarm) -> Vec<Signal> {
        let mut signals = vec![];

        let mut total_score = 0.0;
        let mut count = 0;

        for agent in swarm.agents.values() {
            let agent = agent.read().unwrap();
            if let Some(basic) = agent.as_any().downcast_ref::<crate::agents::basic::BasicReactiveAgent>() {
                total_score += basic.memory.score;
                count += 1;
            }
        }

        if count == 0 {
            return signals;
        }

        let avg = total_score / count as f64;

        if avg > 0.65 {
            signals.push(Signal {
                origin_id: "oracle".into(),
                payload: "global:promote".into(),
            });
        } else if avg < 0.3 {
            signals.push(Signal {
                origin_id: "oracle".into(),
                payload: "global:mutate_more".into(),
            });
        }

        signals
    }
}
