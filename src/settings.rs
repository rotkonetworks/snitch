// settings.rs
// src/settings.rs
use toml;
use serde::Deserialize;
use std::{env, fs};

#[derive(Deserialize, Clone)]
pub struct Config {
    pub server_port: String,
    pub pushover_token: String,
    pub pushover_user: String,
    pub pushover_message: Option<String>,
    pub pushover_priority: Option<String>,
    pub pushover_sound: Option<String>,
    pub alert_check_interval_secs: u64,
    pub api_key_watchdog: Option<String>,
    pub api_key_alert: Option<String>,
    pub postgres_uri: String,
    pub redis_uri: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = env::var("CONFIG_PATH").unwrap_or_else(|_| "config.toml".to_string());
        let config_contents = fs::read_to_string(&config_path)?;
        toml::from_str(&config_contents).map_err(Into::into) // Convert to a generic error
    }
}
