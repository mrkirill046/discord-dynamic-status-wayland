use super::Config;
use super::path::config_path;
use crate::logger::Logger;
use anyhow::Result;
use std::fs;

pub fn load() -> Result<Config> {
    let path = config_path()?;
    Logger::info(&format!("Loading config: {:?}", path));

    if !path.exists() {
        Logger::debug("Config not found, creating default config...");

        let default = include_str!("default-config.toml");

        fs::write(&path, default)?;
    }

    let data = fs::read_to_string(&path)?;
    let config =
        toml::from_str(&data).map_err(|e| anyhow::anyhow!("Failed to parse config.toml: {}", e))?;

    Logger::debug("Config loaded");

    Ok(config)
}
