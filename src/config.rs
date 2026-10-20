use std::{fs::File, io::BufReader, path::PathBuf};

use anyhow::{Context, Result};
use serde::Deserialize;

pub const CONFIG_PATH: &str = "monorelease.yaml";
pub const NEXT_DIR_PATH: &str = "next";
pub const WAVE_DIR_PATH: &str = "waves";
pub const PROJECTS_DIR_PATH: &str = "projects";

#[derive(Debug, Deserialize, Default)]
pub enum LogLevel {
    #[default]
    Info,
    Warning,
    Error,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[allow(clippy::struct_field_names, unused)]
    pub config_version: u16,
    pub base_path: PathBuf,
    // #[serde(default)]
    // log_level: LogLevel,
}

impl Config {
    pub fn load() -> Result<Self> {
        let file = File::open(CONFIG_PATH).context(format!(
            "could not find main configuration file '{CONFIG_PATH}'"
        ))?;
        let reader = BufReader::new(file);
        let mut config: Self = serde_yaml::from_reader(reader)?;

        config.base_path = std::path::absolute(config.base_path)?;

        Ok(config)
    }
}
