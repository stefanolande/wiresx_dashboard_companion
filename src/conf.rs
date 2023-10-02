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

impl Config {
    pub fn load() -> figment::error::Result<Config> {
        Figment::new().merge(Toml::file("conf.toml")).extract()
    }
}
