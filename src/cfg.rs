use serde::{Deserialize, Serialize};
use vipera::Vipera;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub mod_directory_path: String,
    pub exe_path: Option<String>,
}

impl vipera::Configuration for Config {
    fn vipera() -> vipera::Vipera {
        Vipera::default()
            .set_config_name("config.toml")
            .add_config_path("~/.config/mhw")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mod_directory_path: "~/.config/mhw/mods".to_string(),
            exe_path: None,
        }
    }
}
