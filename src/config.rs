use std::path::Path;

use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Wifi {
    pub device: String,
    pub ssid: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub keymap: String,
    pub timezone: String,
    pub locale: String,
    pub disk_path: String,
    pub hostname: String,
    pub wifi: Option<Wifi>,
}

impl Config {
    #[must_use]
    pub fn load(path: impl AsRef<Path>) -> Self {
        let file = std::fs::read_to_string(path).expect("Failed to read config file");

        toml::from_str(&file).expect("Failed to parse config file")
    }
}
