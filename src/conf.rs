use std::error::Error;
use std::fs::File;
use std::io;
use std::io::ErrorKind;

use figment::{
    providers::{Format, Toml},
    Figment,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub wires_x_log: String,
    pub write_log: String,
    pub refres_interval: usize,
    pub max_log_size: usize,
}

const CONF_FILE_PATH: &'static str = "conf.toml";

impl Config {
    pub fn load() -> Result<Config, Box<dyn Error>> {
        match File::open(CONF_FILE_PATH) {
            Ok(_) => Ok::<(), Box<dyn Error>>(()),
            Err(e) if e.kind() == ErrorKind::NotFound => {
                return Err(Box::new(io::Error::new(
                    ErrorKind::NotFound,
                    format!(
                        "Configuration file {} not found in the same path of the program",
                        CONF_FILE_PATH
                    ),
                )))
            }
            _ => Ok(()),
        }?;
        let figment = Figment::new().merge(Toml::file(CONF_FILE_PATH));
        let config: Config = figment.extract()?;
        Ok(config)
    }
}
