use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub api_port: u16,
    pub capture: CaptureConfig,
    pub firewall: FirewallConfig,
    pub log_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureConfig {
    pub interface: String,
    pub promiscuous: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallConfig {
    pub enabled: bool,
    pub nftables_path: String,
}

impl Config {
    pub fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&content)?;
        Ok(config)
    }
}

