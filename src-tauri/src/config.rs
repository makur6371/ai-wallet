use crate::models::AppConfig;
use anyhow::{Result, Context};
use std::fs;
use std::path::PathBuf;

fn config_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .context("Could not find config directory")?
        .join("ai-wallet");
    fs::create_dir_all(&config_dir)?;
    Ok(config_dir.join("config.json"))
}

pub fn load_config() -> AppConfig {
    let path = match config_path() {
        Ok(p) => p,
        Err(_) => return AppConfig::default(),
    };

    if !path.exists() {
        return AppConfig::default();
    }

    let data = fs::read_to_string(&path).unwrap_or_default();
    serde_json::from_str(&data).unwrap_or_default()
}

pub fn save_config(config: &AppConfig) -> Result<()> {
    let path = config_path()?;
    let data = serde_json::to_string_pretty(config)?;
    fs::write(&path, data)?;
    Ok(())
}
