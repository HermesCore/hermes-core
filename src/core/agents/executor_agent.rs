//! Executor Agent
//! Simulates the outcome of trade execution and returns a result with profit/loss

use crate::core::vault_router::VaultRoutingDecision;
use crate::core::swarm::agent_scores::AgentScoreMap;
use rand::Rng;

pub struct ExecutionResult {
    pub agent_id: String,
    pub signal_id: String,
    pub success: bool,
    pub profit: f64,
}

pub struct ExecutorAgent;

impl ExecutorAgent {
    /// Simulates trade execution outcome for a routing decision.
    pub fn execute(
        decision: &VaultRoutingDecision,
        signal_id: &str,
    ) -> ExecutionResult {
        let mut rng = rand::thread_rng();

        let success = rng.gen_bool(0.8); // 80% chance of success
        let profit = if success {
            rng.gen_range(0.5..3.0) // ETH earned
        } else {
            -rng.gen_range(0.1..1.0) // ETH loss
        };

        // Debug output to verify profit calculation
        println!(
            "[executor] Executed {} for {} → success: {}, profit: {:.4} ETH",
            signal_id, decision.agent_id, success, profit
        );

        ExecutionResult {
            agent_id: decision.agent_id.clone(),
            signal_id: signal_id.to_string(),
            success,
            profit,
        }
    }

    /// Applies the result to the agent scoring map.
    pub fn update_score(
        scores: &mut AgentScoreMap,
        result: &ExecutionResult,
        signal_type: &str,
        now: u64,
    ) {
        scores.update_score(&result.agent_id, signal_type, result.profit, now);
        println!(
            "[executor] Updated score for {}: delta {:.4} → new score: {:.4}",
            result.agent_id,
            result.profit,
            scores.get_score(&result.agent_id, signal_type)
        );
    }
}
