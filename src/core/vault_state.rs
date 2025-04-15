//! Vault State
//! Tracks capital allocated to each agent and simulates capital flow based on execution results

use std::collections::HashMap;

/// Vault manager that tracks per-agent capital
#[derive(Debug, Default)]
pub struct VaultState {
    pub balances: HashMap<String, f64>, // agent_id -> balance in ETH
}

impl VaultState {
    /// Create a new empty vault state
    pub fn new() -> Self {
        Self {
            balances: HashMap::new(),
        }
    }

    /// Get current balance for an agent (defaults to 0.0)
    pub fn get_balance(&self, agent_id: &str) -> f64 {
        *self.balances.get(agent_id).unwrap_or(&0.0)
    }

    /// Add or subtract from an agent's vault
    pub fn adjust_balance(&mut self, agent_id: &str, delta: f64) {
        let balance = self.balances.entry(agent_id.to_string()).or_insert(0.0);
        *balance += delta;  // Add profit or loss to the vault balance
        println!(
            "[vault] Adjusted {}'s balance by {:.4} ETH → new balance: {:.4}",
            agent_id, delta, *balance
        );
    }

    /// Set an initial balance for simulation
    pub fn set_balance(&mut self, agent_id: &str, amount: f64) {
        self.balances.insert(agent_id.to_string(), amount);
    }

    /// Print all vaults (for debug/logging)
    pub fn print_balances(&self) {
        println!("📊 Vault Balances:");
        for (agent_id, amount) in &self.balances {
            println!("  - {}: {:.4} ETH", agent_id, amount);
        }
    }
}
