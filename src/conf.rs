use confy::ConfyError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub wires_x_log: String,
    pub write_log: String,
    pub refres_interval: usize,
    pub max_log_size: usize,
}

pub fn load_conf() -> Result<Config, ConfyError> {
    let cfg: Config = confy::load("wiresx_dashboard_companion", "conf")?;
    println!("{:?}", confy::get_configuration_file_path("wiresx_dashboard_companion", "conf"));
    println!("{:#?}", cfg);
    Ok(cfg)
}