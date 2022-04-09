use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub targets: Vec<String>,
}

pub fn load_config() -> Result<Configuration, &'static str> {
    Config::builder()
        .add_source(File::with_name("settings"))
        .add_source(File::with_name("local/settings"))
        .build()
        .map_err(|_| "Could not locate config")
        .and_then(|c| {
            c.try_deserialize::<Configuration>()
                .map_err(|_| "Failed to parse config")
        })
}
