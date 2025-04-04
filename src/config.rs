use serde::Deserialize;
use std::fs;
use anyhow::Result;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub timezones: Vec<String>,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self> {
        let contents = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }
}

pub fn load_config() -> Result<Config> {
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
    let config_path = home.join(".config").join("gtime").join("config.toml");
    
    if !config_path.exists() {
        return Ok(Config { timezones: vec![] });
    }
    
    Config::from_file(config_path.to_str().unwrap())
} 