use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{self, ErrorKind},
    path::Path,
};

#[derive(Serialize, Deserialize)]
pub struct StnConfig {
    pub last_update_check: u64,
    pub latest_version: Option<String>,
}

impl StnConfig {
    pub fn load(stn_config_dir: &Path) -> Result<Self, io::Error> {
        let config_path = stn_config_dir.join("stn_config.toml");
        let content = match fs::read_to_string(config_path) {
            Ok(content) => content,
            Err(_) => {
                return Ok(StnConfig {
                    last_update_check: 0,
                    latest_version: None,
                })
            } // If file not found, return default config
        };
        let config: StnConfig = toml::from_str(&content)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))?;
        Ok(config)
    }

    pub fn save(&self, stn_config_dir: &Path) -> Result<(), io::Error> {
        fs::create_dir_all(stn_config_dir)?;

        let config_path = stn_config_dir.join("stn_config.toml");
        let content = toml::to_string(self).map_err(|err| {
            io::Error::new(
                ErrorKind::InvalidData,
                format!("Failed to serialize config: {}", err),
            )
        })?;
        fs::write(config_path, content.as_bytes()).map_err(|err| {
            io::Error::new(
                ErrorKind::Other,
                format!("Failed to write config to file: {}", err),
            )
        })
    }
}
