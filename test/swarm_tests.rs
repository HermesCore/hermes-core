// tests/core/swarm_tests.rs

use hermes_core::swarm_agents::{Agent, AgentClass, SwarmAgentRegistry};
use std::collections::HashMap;

#[test]
fn test_add_agent_to_registry() {
    let mut swarm_registry = SwarmAgentRegistry::new();
    let agent = Agent::new("Scout1", AgentClass::Scout);
    swarm_registry.add_agent(agent);

    let retrieved_agent = swarm_registry.get_agent("Scout1");
    assert!(retrieved_agent.is_some());
    assert_eq!(retrieved_agent.unwrap().agent_id, "Scout1");
}

#[test]
fn test_agent_execution_and_evolution() {
    let mut swarm_registry = SwarmAgentRegistry::new();
    let mut agent = Agent::new("Sniper1", AgentClass::Sniper);

    // Execute an action that would increase the agent's score
    let current_time = 100;
    agent.execute("arbitrage", current_time);
    assert!(agent.score > 0.0); // Ensure agent score increased

    // Simulate evolution based on performance
    agent.evolve();
    assert_eq!(agent.class, AgentClass::Specialist); // Ensure promotion to Specialist
}

#[test]
fn test_agent_score_update() {
    let mut swarm_registry = SwarmAgentRegistry::new();
    let mut agent = Agent::new("Sniper1", AgentClass::Sniper);
    swarm_registry.add_agent(agent);

    swarm_registry.update_agent_score("Sniper1", 10.0);
    let updated_agent = swarm_registry.get_agent("Sniper1").unwrap();
    assert_eq!(updated_agent.score, 15.0); // Check updated score after change
}

