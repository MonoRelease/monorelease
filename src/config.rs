use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::logging::LogLevel;

pub const CONFIG_PATH: &str = "monorelease.yaml";
pub const NEXT_DIR_PATH: &str = "next/";
pub const WAVE_DIR_PATH: &str = "waves/";
pub const PROJECTS_DIR_PATH: &str = "projects/";

pub const DEFAULT_BASE_PATH: &str = "releases/";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    #[allow(clippy::struct_field_names, unused)]
    pub config_version: u16,
    pub base_path: PathBuf,
    pub log_level: LogLevel,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            config_version: 1,
            base_path: DEFAULT_BASE_PATH.into(),
            log_level: Default::default(),
        }
    }
}

impl Config {
    #[must_use]
    pub fn load_from_file(path: &Path) -> Result<Self> {
        let file = File::open(path).context(format!("could not find file '{}'", path.display()))?;
        let reader = BufReader::new(file);
        Ok(serde_yaml::from_reader(reader)?)
    }

    #[must_use]
    pub fn load_from_dir(dir: &Path) -> Result<Self> {
        Self::load_from_file(&dir.join(CONFIG_PATH))
    }

    pub fn ensure_file_exists_in_dir(dir: &Path) -> Result<()> {
        let config_path = dir.join(CONFIG_PATH);
        if config_path.exists() {
            info!("existing config file detected at {}", config_path.display());
        } else {
            info!(
                "config file does not exist - creating default at {}",
                config_path.display()
            );
            let config = Config::default();
            let file = File::create(config_path.clone())?;
            serde_yaml::to_writer(file, &config)?;
        }
        Ok(())
    }
}
