//! Hermes Node Entrypoint
//!
//! Initializes system, starts agents, and runs the swarm loop.

mod runtime;

use hermes_core::infra::config::Config;

#[tokio::main]
async fn main() {
    let config = Config::load();
    runtime::start(config).await;
}
