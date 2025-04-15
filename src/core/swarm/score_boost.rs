//! Score Boost module
//! Boosts agent scores based on realized profit, precision, and alpha detection.

// --- Imports ---
// use crate::core::swarm::agent_scores::AgentScore;

/// Boost factor configuration
pub struct BoostFactors {
    pub profit_weight: f64,
    pub confidence_weight: f64,
    pub alpha_bonus: f64,
}

/// Apply a profit-weighted score boost.
///
/// # Arguments
/// * `score` - The agent's current score object.
/// * `profit` - Realized profit from a signal execution.
/// * `confidence` - Signal confidence level [0.0 - 1.0].
/// * `alpha_detected` - Whether this signal originated from internal alpha.
pub fn apply_score_boost(
    score: &mut crate::core::swarm::agent_scores::AgentScore,
    profit: f64,
    confidence: f64,
    alpha_detected: bool,
    factors: &BoostFactors,
) {
    let mut boost = profit * factors.profit_weight;
    boost += confidence * factors.confidence_weight;

    if alpha_detected {
        boost += factors.alpha_bonus;
    }

    score.update(boost, score.last_updated);
}
