use anyhow::Result;
use serde::Deserialize;
use std::fs;

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
    let config_path = dirs::preference_dir().unwrap().join("timezconfig.toml");

    if !config_path.exists() {
        return Ok(Config { timezones: vec![] });
    }

    Config::from_file(config_path.to_str().unwrap())
}
