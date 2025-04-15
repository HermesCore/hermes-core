// src/main.rs

use std::env;

fn main() {
    println!("🚀 HermesCore: Swarm Intelligence Runtime Initializing...");

    // Option 1: Get the rounds from command line arguments, default to 100 if not provided
    let args: Vec<String> = env::args().collect();
    let rounds = if args.len() > 1 {
        args[1].parse::<u64>().unwrap_or(100) // Default to 100 if input is not a valid number
    } else {
        100 // Default value
    };

    // Run a full end-to-end signal routing simulation with dynamic rounds
    hermes_core::core::simulation::signal_loop::simulate_swarm_loop(rounds);

    println!("✅ Swarm signal loop simulation complete.");
}
