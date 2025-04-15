// test/core/vault_router_tests.rs

use hermes_core::vault_router::*;
use hermes_core::swarm_agents::{SwarmAgentRegistry, Agent, AgentClass};
use std::collections::HashMap;

#[test]
fn test_routing_picks_best_agent() {
    let mut registry = SwarmAgentRegistry::new();

    let mut agent1 = Agent::new("A1", AgentClass::Scout);
    let mut agent2 = Agent::new("A2", AgentClass::Sniper);
    agent2.score = 10.0;

    registry.add_agent(agent1);
    registry.add_agent(agent2);

    let mut vault_balances = HashMap::new();
    vault_balances.insert("A1".to_string(), 100.0);
    vault_balances.insert("A2".to_string(), 100.0);

    let cooldowns = HashMap::new();
    let now = 100;

    let signal = OpportunitySignal {
        signal_id: "sig1".to_string(),
        signal_type: "arbitrage".to_string(),
        expected_profit: 5.0,
        confidence: 0.9,
    };

    let result = VaultRouter::route_signal(&signal, &registry, &vault_balances, &cooldowns, now);

    assert!(result.is_some());
    let decision = result.unwrap();
    assert_eq!(decision.agent_id, "A2");
    println!("Routing decision: {:?}", decision);
}

#[test]
fn test_agent_on_cooldown_is_skipped() {
    let mut registry = SwarmAgentRegistry::new();
    let mut agent = Agent::new("A3", AgentClass::Executor);
    agent.last_executed = 95;
    registry.add_agent(agent);

    let mut vault_balances = HashMap::new();
    vault_balances.insert("A3".to_string(), 100.0);

    let mut cooldowns = HashMap::new();
    cooldowns.insert("A3".to_string(), 10); // 10s cooldown

    let signal = OpportunitySignal {
        signal_id: "sig2".to_string(),
        signal_type: "liquidation".to_string(),
        expected_profit: 5.0,
        confidence: 1.0,
    };

    let now = 100;
    let result = VaultRouter::route_signal(&signal, &registry, &vault_balances, &cooldowns, now);
    assert!(result.is_none(), "Agent should be skipped due to cooldown");
}

#[test]
fn test_agent_with_zero_balance_is_skipped() {
    let mut registry = SwarmAgentRegistry::new();
    let agent = Agent::new("A4", AgentClass::Specialist);
    registry.add_agent(agent);

    let vault_balances = HashMap::new(); // No balance
    let cooldowns = HashMap::new();

    let signal = OpportunitySignal {
        signal_id: "sig3".to_string(),
        signal_type: "oracle-arb".to_string(),
        expected_profit: 3.0,
        confidence: 0.7,
    };

    let now = 100;
    let result = VaultRouter::route_signal(&signal, &registry, &vault_balances, &cooldowns, now);
    assert!(result.is_none(), "Agent should be skipped due to zero balance");
}