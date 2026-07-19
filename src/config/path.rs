use crate::constants;
use anyhow::{Result, anyhow};
use directories::ProjectDirs;
use std::path::PathBuf;

pub fn config_path() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from(
        constants::QUALIFIER,
        constants::ORGANIZATION,
        constants::APP_NAME,
    )
    .ok_or_else(|| anyhow!("Cannot determine config directory"))?;

    let config_dir = proj_dirs.config_dir();

    std::fs::create_dir_all(config_dir)?;

    Ok(config_dir.join("config.toml"))
}
