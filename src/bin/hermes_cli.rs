use std::process::Command;
use std::env;

const PHASES: &[(&str, &[&str])] = &[
    ("0", &["signal_loop", "vault_state", "vault_router"]),
    ("1", &["vault_growth_scaler", "scaling_trigger", "swarm_scaler"]),
    ("2", &["sniper_agent", "passive_alpha", "sentinel_agent", "agent_scores"]),
    ("3", &["confidence_tracker", "meta_strategy", "outcome_attribution"]),
    ("4", &["signal_cloaker", "wallet_pool", "bundle_jitter"]),
    ("5", &["execution_simulator", "flashbots_engine", "signal_combiner"]),
    ("6", &["flashloan_aave", "flashloan_router", "profit_forwarder"]),
    ("7", &["agent_rotator", "reputation_transfer", "agent_profile"]),
    ("8", &["chain_adapter", "arbitrum", "optimism"]),
    ("9", &["replay_controller", "mock_vault", "integration_tests"]),
    ("10", &["telemetry", "alerts", "dashboard_feed"]),
    ("11", &["strategy_nft", "governance"]),
    ("12", &["param_mutator", "strategy_experimenter", "swarm_distiller"]),
];

fn list_phases() {
    println!("📦 HermesCore Phases:");
    for (phase, modules) in PHASES {
        println!("  Phase {phase}: {} modules", modules.len());
        for m in *modules {
            println!("    - {m}.rs");
        }
    }
}

fn run_tests_for_phase(phase: &str) {
    let Some((_, modules)) = PHASES.iter().find(|(p, _)| *p == phase) else {
        println!("❌ Unknown phase {phase}");
        return;
    };

    println!("🔬 Running tests for Phase {phase}...");
    for module in *modules {
                println!("  • cargo test --lib -p hermes-core -F {module}");
        let _ = Command::new("cargo")
    .args(&["run", "--bin", "hermes-core"])

            .args(&["test", "--lib"])
            .status()
            .expect("Failed to run cargo test");
    }
}

fn run_phase_runtime(phase: &str) {
    println!("🚀 Running runtime for Phase {phase}...");
    let _ = Command::new("cargo")
        .args(&["run", "--bin", "hermes-core"])
        .status()
        .expect("Failed to run simulation");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage:");
        println!("  hermes_cli list phases");
        println!("  hermes_cli run phase <n>");
        println!("  hermes_cli test phase <n>");
        return;
    }

    match args[1].as_str() {
        "list" => {
            if args.get(2).map(String::as_str) == Some("phases") {
                list_phases();
            }
        }
        "run" => {
            if args.get(2).map(String::as_str) == Some("phase") {
                if let Some(phase) = args.get(3) {
                    run_phase_runtime(phase);
                }
            }
        }
        "test" => {
            if args.get(2).map(String::as_str) == Some("phase") {
                if let Some(phase) = args.get(3) {
                    run_tests_for_phase(phase);
                }
            }
        }
        _ => {
            println!("Unknown command.");
        }
    }
}