use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct VaultSnapshot {
    pub round: u64,
    pub balances: HashMap<String, f64>,
}

#[derive(Debug, Default)]
pub struct VaultGrowthScaler {
    pub history: Vec<VaultSnapshot>,
}

impl VaultGrowthScaler {
    pub fn new() -> Self {
        Self { history: Vec::new() }
    }

    pub fn record_snapshot(&mut self, round: u64, current_balances: &HashMap<String, f64>) {
        let snapshot = VaultSnapshot {
            round,
            balances: current_balances.clone(),
        };
        self.history.push(snapshot);
    }

    pub fn compute_swarm_growth(&self) -> (f64, f64) {
        if self.history.len() < 2 {
            return (0.0, 0.0);
        }
        let prev = &self.history[self.history.len() - 2];
        let latest = &self.history[self.history.len() - 1];
        let prev_total: f64 = prev.balances.values().sum();
        let latest_total: f64 = latest.balances.values().sum();
        let delta = latest_total - prev_total;
        let percent = if prev_total > 0.0 {
            delta / prev_total * 100.0
        } else {
            0.0
        };
        (delta, percent)
    }

    pub fn latest_snapshot(&self) -> Option<&VaultSnapshot> {
        self.history.last()
    }

    pub fn compute_agent_growth(&self) -> HashMap<String, f64> {
        if self.history.len() < 2 {
            return HashMap::new();
        }
        let prev = &self.history[self.history.len() - 2];
        let latest = &self.history[self.history.len() - 1];
        let mut growth_map = HashMap::new();
        for (agent_id, latest_balance) in &latest.balances {
            if let Some(prev_balance) = prev.balances.get(agent_id) {
                let delta = latest_balance - prev_balance;
                let percent = if *prev_balance > 0.0 {
                    delta / prev_balance * 100.0
                } else {
                    0.0
                };
                growth_map.insert(agent_id.clone(), percent);
            }
        }
        growth_map
    }
}