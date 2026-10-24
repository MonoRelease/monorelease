use std::{
    fs::{self, File},
    path::Path,
};

use anyhow::{Context, Result};

use crate::{
    MonoRelease,
    config::{CONFIG_PATH, Config},
};

impl MonoRelease {
    /// Initialises the folder structure.
    #[allow(clippy::todo)]
    pub fn init(dir: &Path) -> Result<()> {
        println!("initialising at {}", dir.display()); // TODO: Replace with logging
        let config_path = dir.join(CONFIG_PATH);

        if config_path.exists() {
            println!("existing config file detected at {}", config_path.display());
        } else {
            println!(
                "config file does not exist, creating default at {}",
                config_path.display()
            );
            let config = Config::default();
            let file = File::create(config_path.clone())?;
            serde_yaml::to_writer(file, &config)?;
        }

        let config = Config::load(&config_path).context("failed to load configuration")?;
        let monorelease = Self::new(config);
        let config = &monorelease.config;

        fs::create_dir_all(config.base_path.clone())?;
        let dirs_to_create = [
            crate::config::NEXT_DIR_PATH,
            crate::config::PROJECTS_DIR_PATH,
            crate::config::WAVE_DIR_PATH,
        ];
        for dir_path in dirs_to_create {
            let path = dir.join(&config.base_path).join(dir_path);
            println!("Creating {}", path.display()); // TODO: Replace with logging
            fs::create_dir_all(path)?;
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
        let root_path = temp_dir.path().to_path_buf();

        MonoRelease::init(&root_path)?;

        let base_path = root_path.join(crate::config::DEFAULT_BASE_PATH);

        let paths_to_check_exists = [
            root_path.join(crate::config::CONFIG_PATH),
            base_path.join(crate::config::NEXT_DIR_PATH),
            base_path.join(crate::config::PROJECTS_DIR_PATH),
            base_path.join(crate::config::WAVE_DIR_PATH),
        ];

        for path in paths_to_check_exists {
            println!("Checking {}", base_path.join(&path).display());
            assert!(base_path.join(&path).exists());
        }

        Ok(())
    }
}
