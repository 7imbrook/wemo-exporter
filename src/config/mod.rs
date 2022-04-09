use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub targets: Vec<String>,
}

pub fn load_config() -> Result<Configuration, &'static str> {
    Config::builder()
        .add_source(File::with_name("settings"))
        .build()
        .unwrap()
        .try_deserialize::<Configuration>()
        .map_err(|_| "Failed to parse config")
}
