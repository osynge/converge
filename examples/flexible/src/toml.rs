use crate::errors::ConfigureErr;
use crate::Config;
use serde_derive::Deserialize;
use std::path::Path;

#[derive(Deserialize)]
pub struct ConfigFile {
    pub loglevel: Option<i8>,
    pub xunit: Option<Vec<String>>,
    pub environment_sk: Option<String>,
    pub environment_keys: Option<Vec<String>>,
    pub project_sk: Option<String>,
    pub project_identifier: Option<String>,
    pub project_human_name: Option<String>,
    pub service_url: Option<String>,
}

pub(crate) fn load_config_from_path_string(input_path: &String) -> Result<Config, ConfigureErr> {
    let path = Path::new(input_path);
    if !path.is_file() {
        return Err(ConfigureErr::FilePathIsNotFile(String::from(input_path)));
    }
    let toml_str = std::fs::read_to_string(&path)?;
    let cf: ConfigFile = toml::from_str(&toml_str)?;
    Ok(cf.into())
}

pub fn load_config_from_default_path() -> Result<Config, ()> {
    let path = String::from("/etc/xunit-repo-client.toml");
    if let Ok(cfg) = load_config_from_path_string(&path) {
        return Ok(cfg);
    };
    if let Some(mut dirhome) = dirs::home_dir() {
        dirhome.push(".xunit-repo-client.toml");
        if let Some(cfg_path_str) = dirhome.to_str() {
            let cfg_path = String::from(cfg_path_str);
            if let Ok(cfg) = load_config_from_path_string(&cfg_path) {
                return Ok(cfg);
            };
        }
    }
    Err(())
}
