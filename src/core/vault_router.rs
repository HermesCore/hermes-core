// core/vault_router.rs

use std::collections::HashMap;
use crate::core::agents::swarm_agent::{Agent, AgentClass, SwarmAgentRegistry};

/// Represents a signal detected in the market
#[derive(Debug, Clone)]
pub struct OpportunitySignal {
    pub signal_id: String,
    pub signal_type: String,
    pub expected_profit: f64,
    pub confidence: f64,
}

/// Routing decision for a selected agent
#[derive(Debug, Clone)]
pub struct VaultRoutingDecision {
    pub agent_id: String,
    pub allocated_amount: f64,
    pub reason: String,
}

/// Filters used to guide routing decisions (can be extended later)
#[derive(Clone)]
pub struct VaultRouterFilters {
    pub cooldowns: HashMap<String, u64>,
    pub vault_balances: HashMap<String, f64>,
}

/// Capital allocator that routes signals to the best agent
pub struct VaultRouter {
    scores: HashMap<String, f64>,
    signal_type: String,
    confidence_threshold: f64,
    filters: VaultRouterFilters,
    timestamp: u64,
    max_allocation: f64,
    registry: SwarmAgentRegistry,
}

impl VaultRouter {
    pub fn new(
        scores: HashMap<String, f64>,
        registry: SwarmAgentRegistry,
        signal_type: &str,
        confidence_threshold: f64,
        filters: VaultRouterFilters,
        timestamp: u64,
        max_allocation: f64,
    ) -> Self {
        Self {
            scores,
            registry,
            signal_type: signal_type.to_string(),
            confidence_threshold,
            filters,
            timestamp,
            max_allocation,
        }
    }

    /// Routes the top `max` agents for a given signal
    pub fn route(&self, max: usize) -> Vec<VaultRoutingDecision> {
        let mut decisions = Vec::new();

        for agent in self.registry.get_all_agents() {
            if let Some(balance) = self.filters.vault_balances.get(&agent.agent_id) {
                if *balance <= 0.0 {
                    continue;
                }
            }

            if let Some(cooldown) = self.filters.cooldowns.get(&agent.agent_id) {
                if self.timestamp < agent.last_executed + cooldown {
                    continue;
                }
            }

            let specialization_multiplier = match agent.class {
                AgentClass::Sniper => 1.2,
                AgentClass::Specialist => 1.1,
                AgentClass::Scout => 1.0,
                AgentClass::Executor => 1.3,
                AgentClass::Sentinel => 0.8,
            };

            let signal_confidence = 0.8; // TODO: use real value per signal
            let expected_profit = 1.0;   // TODO: inject real profit estimate

            let score = signal_confidence * expected_profit * specialization_multiplier;

            if score >= self.confidence_threshold {
                let agent_bias = match agent.class {
                    AgentClass::Executor => 1.5,
                    AgentClass::Specialist => 1.3,
                    AgentClass::Sniper => 1.2,
                    AgentClass::Scout => 1.0,
                    AgentClass::Sentinel => 0.5,
                };

                let base_allocation = expected_profit * signal_confidence * 10.0;
                let allocation = (base_allocation * agent_bias)
                    .min(*self.filters.vault_balances.get(&agent.agent_id).unwrap_or(&0.0))
                    .min(self.max_allocation);

                let decision = VaultRoutingDecision {
                    agent_id: agent.agent_id.clone(),
                    allocated_amount: allocation,
                    reason: format!(
                        "score={:.2}, class={:?}, bias={:.1}",
                        score, agent.class, agent_bias
                    ),
                };

                decisions.push(decision);
            }
        }

        decisions.sort_by(|a, b| b.allocated_amount.partial_cmp(&a.allocated_amount).unwrap());
        decisions.truncate(max);
        decisions
    }
}

/// Stores a history of all routing decisions made by VaultRouter
#[derive(Debug, Default)]
pub struct VaultRoutingHistory {
    pub decisions: Vec<VaultRoutingDecision>,
}

impl VaultRoutingHistory {
    pub fn new() -> Self {
        Self {
            decisions: Vec::new(),
        }
    }

    pub fn log_decision(&mut self, decision: VaultRoutingDecision) {
        println!(
            "📝 [VaultRouter Log] Routed to: {}, Amount: {:.2}, Reason: {}",
            decision.agent_id, decision.allocated_amount, decision.reason
        );
        self.decisions.push(decision);
    }

    pub fn print_history(&self) {
        println!("\n📊 Vault Routing History Log:");
        for (i, decision) in self.decisions.iter().enumerate() {
            println!(
                "  [{}] Agent: {}, Amount: {:.2}, Reason: {}",
                i + 1,
                decision.agent_id,
                decision.allocated_amount,
                decision.reason
            );
        }
    }
}