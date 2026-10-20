use std::path::PathBuf;

use anyhow::Result;
use serde::Deserialize;

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
        // TODO: Convert the base path to an absolute path
        todo!()
    }
}
