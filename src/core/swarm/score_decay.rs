//! Score Decay module
//! Applies time-based decay to agent scores to ensure that recent performance matters most.

// --- Imports ---
use crate::core::swarm::agent_scores::{AgentScore, AgentScoreMap};

/// Applies exponential decay to a score value.
///
/// # Arguments
/// * `current_score` - The agent's current score.
/// * `decay_rate` - A multiplier < 1.0 (e.g. 0.99 per time unit).
pub fn decay_score(current_score: f64, decay_rate: f64) -> f64 {
    current_score * decay_rate
}

/// Decays a full `AgentScore` object by a given rate.
pub fn apply_decay_to_agent(score: &mut AgentScore, decay_rate: f64) {
    score.apply_decay(decay_rate);
}

/// Decays all agent scores in a `AgentScoreMap` by a uniform rate.
pub fn decay_all_agents(score_map: &mut AgentScoreMap, decay_rate: f64) {
    for agent_scores in score_map.scores.values_mut() {
        for score in agent_scores.values_mut() {
            apply_decay_to_agent(score, decay_rate);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::swarm::agent_scores::{AgentScore, AgentScoreMap};

    #[test]
    fn test_decay_individual_score() {
        let mut score = AgentScore::new();
        score.value = 10.0;
        apply_decay_to_agent(&mut score, 0.9);
        assert!((score.value - 9.0).abs() < 0.001);
    }

    #[test]
    fn test_decay_all_agents() {
        let mut scores = AgentScoreMap::new();
        scores.update_score("agent1", "signalA", 10.0, 1000);
        scores.update_score("agent2", "signalA", 20.0, 1000);

        decay_all_agents(&mut scores, 0.5);

        assert!((scores.get_score("agent1", "signalA") - 5.0).abs() < 0.001);
        assert!((scores.get_score("agent2", "signalA") - 10.0).abs() < 0.001);
    }
}
