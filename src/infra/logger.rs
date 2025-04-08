//! Logger
//!
//! Initializes and manages structured logging across Hermes Core.

use std::env;
use tracing_subscriber::{fmt, EnvFilter};

pub fn init_logger() {
    let default_filter = "hermes=info";

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(default_filter));

    fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_thread_names(true)
        .with_line_number(true)
        .compact()
        .init();
}
