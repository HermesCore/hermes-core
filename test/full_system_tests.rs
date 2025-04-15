// test/integration/full_system_tests.rs

use hermes_core::swarm_agents::{Agent, AgentClass, SwarmAgentRegistry};
use hermes_core::swarm::signal_buffer::{BufferedSignal, SignalBuffer};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn test_full_system_integration() {
    // Step 1: Initialize the Swarm Agent Registry
    let mut swarm_registry = SwarmAgentRegistry::new();

    // Step 2: Create and add agents to the registry
    let mut scout_agent = Agent::new("Scout1", AgentClass::Scout);
    let mut sniper_agent = Agent::new("Sniper1", AgentClass::Sniper);
    swarm_registry.add_agent(scout_agent);
    swarm_registry.add_agent(sniper_agent);

    // Step 3: Simulate agent execution and evolution
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    swarm_registry.execute_agent("Scout1", "arbitrage", current_time);
    swarm_registry.execute_agent("Sniper1", "liquidation", current_time);

    // Ensure agents evolve after executing actions (e.g., upgrading or downgrading classes)
    let scout = swarm_registry.get_agent("Scout1").unwrap();
    let sniper = swarm_registry.get_agent("Sniper1").unwrap();

    assert_eq!(scout.class, AgentClass::Scout);
    assert_eq!(sniper.class, AgentClass::Specialist); // Assuming evolution logic after execution

    // Step 4: Simulate signal buffer and prune expired signals
    let mut signal_buffer = SignalBuffer::new();
    let buffered_signal = BufferedSignal {
        signal_id: "signal1".to_string(),
        timestamp: current_time,
        ttl_seconds: 30,
    };
    signal_buffer.insert(buffered_signal);

    // Step 5: Test signal expiration
    signal_buffer.prune(current_time + 40); // Expiring signals

    assert_eq!(signal_buffer.active_count(current_time + 40), 0); // No active signals after pruning

    // Step 6: Ensure no duplicates are detected for new signals
    signal_buffer.insert(BufferedSignal {
        signal_id: "signal2".to_string(),
        timestamp: current_time + 50,
        ttl_seconds: 20,
    });

    assert_eq!(signal_buffer.active_count(current_time + 60), 1); // Only one signal should be active
}
