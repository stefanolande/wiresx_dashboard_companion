use std::error::Error;
use std::io;
use std::io::ErrorKind;
use std::path::Path;

use figment::{
    providers::{Format, Toml},
    Figment,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub wires_x_log: String,
    pub write_log: String,
    pub refresh_interval: usize,
    pub max_log_size: usize,
    pub show_startup_message: bool,
}

const CONF_FILE_PATH: &'static str = "conf.toml";

impl Config {
    pub fn load() -> Result<Config, Box<dyn Error>> {
        if Path::new(CONF_FILE_PATH).exists() {
            let figment = Figment::new().merge(Toml::file(CONF_FILE_PATH));
            let config: Config = figment.extract()?;
            Ok(config)
        } else {
            return Err(Box::new(io::Error::new(
                ErrorKind::NotFound,
                format!(
                    "Configuration file {} not found in the same path of the program",
                    CONF_FILE_PATH
                ),
            )));
        }
    }
}
