use std::{
    fs::{self, File},
    path::Path,
};

use anyhow::Result;

use crate::{
    MonoRelease,
    config::{CONFIG_PATH, Config},
};

impl MonoRelease {
    /// Initialises the folder structure.
    #[allow(clippy::todo)]
    pub fn init(dir: &Path) -> Result<()> {
        let config_path = dir.join(CONFIG_PATH);
        if !config_path.exists() {
            let config = Config::default();
            let file = File::create(config_path.clone())?;
            serde_yaml::to_writer(file, &config)?;
        }

        let config = Config::load(&config_path)?;
        let monorelease = MonoRelease::new(config);
        let config = &monorelease.config;

        fs::create_dir_all(config.base_path.clone())?;
        let dirs_to_create = [
            crate::config::NEXT_DIR_PATH,
            crate::config::PROJECTS_DIR_PATH,
            crate::config::WAVE_DIR_PATH,
        ];
        for dir_path in dirs_to_create {
            fs::create_dir_all(dir.join(&config.base_path).join(dir_path))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Context, Result};
    use tempfile::tempdir;

    #[test]
    fn test_init_creates_directories() -> Result<()> {
        let temp_dir = tempdir().context("Failed to create temp dir")?;
        let base_path = temp_dir.path().to_path_buf();

        let _ = MonoRelease::init(&base_path);

        assert!(base_path.join(crate::config::NEXT_DIR_PATH).exists());
        assert!(base_path.join(crate::config::PROJECTS_DIR_PATH).exists());
        assert!(base_path.join(crate::config::WAVE_DIR_PATH).exists());

        Ok(())
    }
}
