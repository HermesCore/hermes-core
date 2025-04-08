use crate::{Order, Action, ExecutionResult};
use crate::execution::bundle::BundleExecutor;

pub struct Simulator;

impl Simulator {
    pub fn new() -> Self {
        Self
    }

    fn simulate_bundle(&self, orders: &[Order]) -> ExecutionResult {
        let gas_per_order = 21000;
        let gas_used = gas_per_order * orders.len() as u64;
        let total_volume: f64 = orders.iter().map(|o| o.amount).sum();

        ExecutionResult {
            success: true,
            gas_used,
            tx_hash: Some(format!("sim-tx-{}", orders.len())),
            notes: Some(format!(
                "Simulated {}-hop bundle, total volume {:.4}, gas {}",
                orders.len(),
                total_volume,
                gas_used
            )),
        }
    }
}

impl BundleExecutor for Simulator {
    fn simulate_profit(&self, orders: &[Order]) -> f64 {
        orders.iter().map(|o| 1.0 - o.fee).product::<f64>() - 1.0
    }

    fn execute_bundle(&self, orders: &[Order]) -> ExecutionResult {
        self.simulate_bundle(orders)
    }
}
