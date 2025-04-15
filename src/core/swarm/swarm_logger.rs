use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use crate::core::agents::swarm_agent::Agent;
use crate::core::vault_growth_scaler::VaultSnapshot;
use chrono::Utc;

pub struct SwarmLogger;

impl SwarmLogger {
    pub fn log_scaling_event(
        round: u64,
        action: &str,
        count: usize,
        agent_list: &[&Agent],
        snapshot: &VaultSnapshot,
    ) {
        let file_path = "swarm_scaling_log.csv";
        let exists = Path::new(file_path).exists();

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)
            .expect("Failed to open swarm_scaling_log.csv");

        if !exists {
            writeln!(
                file,
                "timestamp,round,action,count,agent_id,class,balance,growth_score,parent_id,generation"
            )
            .unwrap();
        }

        let timestamp = Utc::now().to_rfc3339();

        for agent in agent_list {
            let growth = snapshot.balances.get(&agent.agent_id).unwrap_or(&0.0);
            writeln!(
                file,
                "{},{},{},{},{},{},{:.4},{:.4},{},{}",
                timestamp,
                round,
                action,
                count,
                agent.agent_id,
                format!("{:?}", agent.class),
                agent.vault_balance,
                growth,
                agent.parent_id.clone().unwrap_or_else(|| "None".to_string()),
                agent.generation
            )
            .unwrap();
        }
    }
}