use serde::Deserialize;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub init_commands: Vec<String>,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_file = std::fs::read_to_string("config.toml")?;
    let config = toml::from_str(&config_file)?;
    Ok(config)
}