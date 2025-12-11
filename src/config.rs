use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Hardcoded wallpaper repository URL
pub const WALLPAPER_REPO: &str = "https://github.com/Incalculas/wallpapers";

/// Configuration structure
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub wallpaper_dir: PathBuf,
    #[serde(default = "default_cycle_interval")]
    pub cycle_interval: u64,
}

fn default_cycle_interval() -> u64 {
    300 // 5 minutes
}

/// Get the path to the config file based on OS
pub fn get_config_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir().context("Could not find config directory")?;
    let app_config_dir = config_dir.join("wcapp");
    fs::create_dir_all(&app_config_dir).context("Failed to create config directory")?;
    Ok(app_config_dir.join("config.toml"))
}

/// Load configuration from file
pub fn load_config() -> Option<Config> {
    let config_path = get_config_path().ok()?;
    if !config_path.exists() {
        return None;
    }

    let content = fs::read_to_string(config_path).ok()?;
    toml::from_str(&content).ok()
}

/// Save configuration to file
pub fn save_config(config: &Config) -> Result<()> {
    let config_path = get_config_path()?;
    let content = toml::to_string(config).context("Failed to serialize config")?;
    fs::write(config_path, content).context("Failed to write config file")?;
    Ok(())
}

/// Get the wallpaper directory from config or default
pub fn get_wallpaper_dir() -> Result<PathBuf> {
    if let Some(config) = load_config() {
        return Ok(config.wallpaper_dir);
    }

    let pictures_dir = dirs::picture_dir().context("Could not find Pictures directory")?;
    Ok(pictures_dir.join("wcapp"))
}
