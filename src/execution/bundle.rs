use crate::{Order, ExecutionResult};

/// The bundle executor trait defines logic for executing or simulating a group of orders.
pub trait BundleExecutor: Send + Sync {
    fn simulate_profit(&self, orders: &[Order]) -> f64;
    fn execute_bundle(&self, orders: &[Order]) -> ExecutionResult;
}

/// Utility function to estimate profit from a sequence of orders.
/// Assumes each order has a `fee` field, applies cumulative fee impact.
pub fn estimate_profit(orders: &[Order]) -> f64 {
    let mut volume = 1.0;
    for order in orders {
        let fee_adj = 1.0 - order.fee;
        volume *= fee_adj;
    }
    volume - 1.0
}
