use crate::core::types::SwarmSignal;
use crate::core::swarm::signal_buffer::{SignalBuffer, BufferedSignal};
use crate::core::vault_router::{VaultRouter, VaultRouterFilters, VaultRoutingDecision};
use crate::core::vault_state::VaultState;
use crate::core::swarm::cooldown_manager::CooldownManager;
use crate::core::vault_growth_scaler::VaultGrowthScaler;
use crate::core::scaling_trigger::{ScalingTrigger, ScalingDecision};
use crate::core::swarm::swarm_scaler::SwarmScaler;
use crate::core::swarm::swarm_state::SwarmState;
use crate::core::agents::swarm_agent::{AgentClass, Agent};

use std::collections::{HashMap, HashSet};

pub fn simulate_swarm_loop(rounds: u64) {
    let signal_types = vec!["liquidation", "arbitrage", "oracle-arb"];
    let now = 1000;

    let mut signal_buffer = SignalBuffer::new();
    let mut vault_state = VaultState::new();
    let mut swarm_state = SwarmState::new();
    let mut scaler = VaultGrowthScaler::new();
    let trigger = ScalingTrigger::new(5.0, -2.0);

    for i in 0..200 {
        let class = match i % 5 {
            0 => AgentClass::Scout,
            1 => AgentClass::Sniper,
            2 => AgentClass::Specialist,
            3 => AgentClass::Executor,
            _ => AgentClass::Sentinel,
        };
        let agent = Agent::new(&format!("agent{:03}", i + 1), class);
        vault_state.set_balance(&agent.agent_id, agent.vault_balance);
        swarm_state.add_agent(agent);
    }

    let mut cooldown_map = HashMap::new();
    for agent_id in swarm_state.agents.keys() {
        cooldown_map.insert(agent_id.clone(), 60);
    }

    let filters = VaultRouterFilters {
        cooldowns: cooldown_map.clone(),
        vault_balances: vault_state.balances.clone(),
    };

    let signals: Vec<SwarmSignal> = signal_types
        .iter()
        .enumerate()
        .map(|(i, ty)| SwarmSignal {
            signal_id: format!("sig-{}", i),
            signal_type: ty.to_string(),
            confidence: 0.8,
            timestamp: now,
            ttl_seconds: 60,
        })
        .collect();

    for round in 1..=rounds {
        println!("\n🔁 Round {}/{} ---------------------------", round, rounds);

        for signal in &signals {
            if signal_buffer.is_duplicate(&signal.signal_id, now) {
                continue;
            }

            signal_buffer.insert(BufferedSignal {
                signal_id: signal.signal_id.clone(),
                timestamp: signal.timestamp,
                ttl_seconds: signal.ttl_seconds,
            });

            let scores: HashMap<String, f64> = swarm_state
                .agents
                .iter()
                .map(|(id, agent)| (id.clone(), agent.score))
                .collect();

            let registry = crate::core::agents::swarm_agent::SwarmAgentRegistry::from_state(&swarm_state);

            let router = VaultRouter::new(
                scores,
                registry,
                &signal.signal_type,
                0.5,
                filters.clone(),
                now,
                50.0,
            );

            let routes: Vec<VaultRoutingDecision> = router.route(5);

            for route in &routes {
                println!(
                    "✅ Routed {} → {} (cap: {:.2}) | {}",
                    signal.signal_id,
                    route.agent_id,
                    route.allocated_amount,
                    route.reason
                );
            }

            for route in &routes {
                if let Some(agent) = swarm_state.agents.get_mut(&route.agent_id) {
                    agent.execute(&signal.signal_type, now);
                }
            }
        }

        scaler.record_snapshot(round, &vault_state.balances);

        if let Some(growth_snapshot) = scaler.latest_snapshot() {
            let decision = trigger.evaluate(&scaler);
            println!("📈 Scaling Decision: {:?}", decision);
            SwarmScaler::apply_scaling_decision(round, &mut swarm_state, decision, growth_snapshot);
        }

        println!("📊 Vault Snapshot After Round {}:", round);
        vault_state.print_balances();
    }

    println!("\n✅ Swarm simulation complete.");
}