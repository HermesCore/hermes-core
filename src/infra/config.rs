//! Config Loader
//!
//! Loads Hermes configuration from .env and config.yaml.

use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub network: String,
    pub default_threshold: f64,
    pub strategy: String,
    pub rpc_url: String,
    pub flashbots_relay: String,
    pub private_key: String,
    pub log_level: Option<String>,
    pub research_mode: Option<bool>,
    pub restore_snapshot: Option<String>,
}

impl Config {
    pub fn load() -> Self {
        dotenvy::dotenv().ok();

        let path = Path::new("config.yaml");
        let file_contents = fs::read_to_string(path).unwrap_or_else(|_| "".into());

        let mut config: Config = serde_yaml::from_str(&file_contents)
            .unwrap_or_else(|_| Config {
                network: "goerli".into(),
                default_threshold: 2000.0,
                strategy: "multi_dex".into(),
                rpc_url: std::env::var("RPC_URL").unwrap_or_default(),
                flashbots_relay: std::env::var("FLASHBOTS_RELAY").unwrap_or_default(),
                private_key: std::env::var("PRIVATE_KEY").unwrap_or_default(),
                log_level: Some("info".into()),
                research_mode: Some(false),
                restore_snapshot: None,
            });

        if let Ok(rpc_url) = std::env::var("RPC_URL") {
            config.rpc_url = rpc_url;
        }

        if let Ok(key) = std::env::var("PRIVATE_KEY") {
            config.private_key = key;
        }

        if let Ok(relay) = std::env::var("FLASHBOTS_RELAY") {
            config.flashbots_relay = relay;
        }

        config
    }
}
