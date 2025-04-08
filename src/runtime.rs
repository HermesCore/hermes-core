//! Hermes Runtime
//!
//! Composes the Hermes system and runs the agent swarm.

use hermes_core::infra::clock::Clock;
use hermes_core::infra::config::Config;
use hermes_core::infra::logger;
use hermes_core::swarm::Swarm;
use hermes_core::agents::basic::BasicReactiveAgent;
use hermes_core::arbitrage::strategies::multi_dex::MultiDexArb;
use std::sync::{Arc, RwLock};

pub async fn start(config: Config) {
    logger::init_logger();
    tracing::info!("Starting Hermes Swarm...");

    let mut swarm = Swarm::new();

    let strategy = Arc::new(MultiDexArb::new());

    // Example agent registration
    for i in 0..config.default_threshold as usize {
        let mut agent = BasicReactiveAgent::new(&format!("agent-{}", i), 2000.0);
        agent.set_strategy(strategy.clone());
        swarm.register_agent(Arc::new(RwLock::new(agent)));
    }

    Clock::start_loop(1000, move |timestamp| {
        swarm.tick_all(timestamp);
        let actions = swarm.collect_actions();

        for (id, action) in actions {
            tracing::info!(%id, ?action, "Agent action at tick");
        }
    })
    .await;
}
