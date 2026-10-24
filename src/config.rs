use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

pub const CONFIG_PATH: &str = "monorelease.yaml";
pub const NEXT_DIR_PATH: &str = "next/";
pub const WAVE_DIR_PATH: &str = "waves/";
pub const PROJECTS_DIR_PATH: &str = "projects/";

pub const DEFAULT_BASE_PATH: &str = "releases/";

#[derive(Debug, Deserialize, Serialize, Default)]
pub enum LogLevel {
    #[default]
    Info,
    Warning,
    Error,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    #[allow(clippy::struct_field_names, unused)]
    pub config_version: u16,
    pub base_path: PathBuf,
    // #[serde(default)]
    // log_level: LogLevel,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            config_version: 1,
            base_path: DEFAULT_BASE_PATH.into(),
        }
    }
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        let file = File::open(path).context(format!("could not find file '{}'", path.display()))?;
        let reader = BufReader::new(file);
        Ok(serde_yaml::from_reader(reader)?)
    }
}
