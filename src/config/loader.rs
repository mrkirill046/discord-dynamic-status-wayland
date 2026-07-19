use super::Config;
use super::path::config_path;
use crate::logger::Logger;
use anyhow::Result;
use std::fs;

pub fn load() -> Result<Config> {
    let path = config_path()?;

    if !path.exists() {
        Logger::debug("Config not found, creating default config...");

        let default = include_str!("default-config.json");

        fs::write(&path, default)?;
    }

    let data = fs::read_to_string(&path)?;
    let config = serde_json::from_str(&data)?;

    Logger::debug("Config loaded");

    Ok(config)
}
