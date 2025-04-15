use crate::core::types::AgentId;
use crate::core::scaling_trigger::ScalingDecision;
use crate::core::vault_growth_scaler::VaultSnapshot;
use crate::core::agents::swarm_agent::{HermesAgentFactory, Agent};
use crate::core::swarm::swarm_state::SwarmState;
use crate::core::swarm::swarm_logger::SwarmLogger;

pub struct SwarmScaler;

impl SwarmScaler {
    pub fn apply_scaling_decision(
        round: u64,
        swarm_state: &mut SwarmState,
        decision: ScalingDecision,
        growth_snapshot: &VaultSnapshot,
    ) {
        match decision {
            ScalingDecision::Grow(n) => {
                let mut new_agents = Vec::new();
                for _ in 0..n {
                    let new_agent = HermesAgentFactory::create_random();
                    swarm_state.add_agent(new_agent.clone());
                    new_agents.push(new_agent);
                }
                let agent_refs: Vec<&Agent> = new_agents.iter().collect();
                SwarmLogger::log_scaling_event(round, "Grow", n, &agent_refs, growth_snapshot);
            }

            ScalingDecision::Shrink(n) => {
                let to_remove = swarm_state.get_lowest_performers(n, growth_snapshot);
                let mut removed_agents = Vec::new();
                for agent_id in &to_remove {
                    if let Some(agent) = swarm_state.agents.get(agent_id) {
                        removed_agents.push(agent.clone());
                    }
                    swarm_state.remove_agent(agent_id);
                }
                let agent_refs: Vec<&Agent> = removed_agents.iter().collect();
                SwarmLogger::log_scaling_event(round, "Shrink", n, &agent_refs, growth_snapshot);
            }

            ScalingDecision::Hold => {
                println!(
                    "[SwarmScaler] HOLD → Swarm size unchanged ({})",
                    swarm_state.agent_count()
                );
            }
        }
    }
}