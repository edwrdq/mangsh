use std::{fs, path::PathBuf};

use anyhow::Result;

use super::Config;
use std::fs::File;
use std::io::Write;

pub struct LoadedConfig {
    pub config: Config,
    pub path: Option<PathBuf>,
    pub error: Option<String>,
}

pub fn default_config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|dir| dir.join("mangsh").join("config.toml"))
}

pub fn load_config() -> LoadedConfig {
    let path = default_config_path();
    let mut loaded = LoadedConfig {
        config: Config::default(),
        path: path.clone(),
        error: None,
    };

    if let Some(path) = path {
        match fs::read_to_string(&path) {
            Ok(contents) => match parse_config(&contents) {
                Ok(cfg) => loaded.config = cfg,
                Err(err) => loaded.error = Some(format!("Unable to parse config: {err}")),
            },
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                loaded.error = Some("Config not found; using defaults.".to_string());
            }
            Err(err) => {
                loaded.error = Some(format!("Unable to read config: {err}"));
            }
        }
    } else {
        loaded.error = Some("No config directory available; using defaults.".to_string());
    }

    loaded
}

fn parse_config(contents: &str) -> Result<Config> {
    let cfg: Config = toml::from_str(contents)?;
    Ok(cfg)
}

pub fn save_config(config: &Config) -> Result<Option<PathBuf>> {
    if let Some(path) = default_config_path() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = File::create(&path)?;
        let serialized = toml::to_string_pretty(config)?;
        file.write_all(serialized.as_bytes())?;
        Ok(Some(path))
    } else {
        Ok(None)
    }
}
