use config::{Config, File, Environment};
use serde::Deserialize;
use std::error::Error;
use config::ConfigError;

#[derive(Debug, Deserialize)]
pub struct CricketConfig {
    pub api_url: String,
    pub api_key: String,
    pub interval: u64,
    pub submission_interval: u64,
    pub key_path: String,
}

impl TryFrom<Config> for CricketConfig {
    type Error = ConfigError;

    fn try_from(config: Config) -> Result<Self, Self::Error> {
        config.try_deserialize()
    }
}

impl CricketConfig {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut config = Config::default();
        config.merge(File::with_name("config/default"))?;
        config.merge(Environment::with_prefix("CRICKET"))?;
        let cricket_config: CricketConfig = config.try_into()?;
        Ok(cricket_config)
    }
}
