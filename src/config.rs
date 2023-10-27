use std::fs;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub postgress: String,

    pub port: u16,
}

impl Config {
    pub(crate) fn from_file(path: &str) -> anyhow::Result<Config> {
        Ok(toml::from_str(fs::read_to_string(path)?.as_str())?)
    }
}
