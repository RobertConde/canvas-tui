use std::{
    fs, io,
    path::{Path, PathBuf},
    sync::OnceLock,
};
use inquire_derive::Selectable;
use strum::Display;
use toml;

use serde::{Deserialize, Serialize};

use crate::canvas::config::CanvasConfig;

#[derive(Debug, thiserror::Error)]
pub enum TuiConfigReadError {
    #[error("failed to open config file")]
    FileOpenError(#[from] io::Error),
    #[error("failed to deserialize config")]
    DeserializeError(#[from] toml::de::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum TuiConfigWriteError {
    #[error("failed to write config file")]
    FileWriteError(#[from] io::Error),
    #[error("failed to serialize config")]
    SerializeError(#[from] toml::ser::Error),
}


#[derive(Debug, Clone, Copy, Selectable, Display)]
pub enum TuiConfigOption {
    #[strum(to_string = "Canvas Configuration")]
    CanvasConfig
}

#[derive(Deserialize, Serialize)]
pub struct TuiConfig {
    #[serde(rename = "canvas")]
    pub canvas_config: Option<CanvasConfig>,
}

static DEFAULT_CONFIG_PATH: OnceLock<PathBuf> = OnceLock::new();
pub const DEFAULT_CONFIG_FILENAME: &str = ".canvas_tui.toml";

pub fn default_config_path() -> &'static PathBuf {
    DEFAULT_CONFIG_PATH.get_or_init(|| {
        dirs::home_dir()
            .unwrap_or(std::env::current_dir().unwrap())
            .join(DEFAULT_CONFIG_FILENAME)
    })
}

impl TuiConfig {
    pub fn load(path: &Path) -> Result<Self, TuiConfigReadError> {
        let content = fs::read_to_string(path)?;

        Ok(toml::from_str(&content)?)
    }

    pub fn write_config(&self, path: &Path) -> Result<(), TuiConfigWriteError> {
        Ok(fs::write(path, toml::to_string_pretty(self)?)?)
    }
}
