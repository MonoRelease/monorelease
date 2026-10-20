use std::fs;

use anyhow::Result;

use crate::{MonoRelease, config::Config};

impl MonoRelease {
    /// Initialises the folder structure.
    #[allow(clippy::todo)]
    pub fn init(&self) -> Result<()> {
        let config = Config::load()?;

        fs::create_dir_all(config.base_path.clone())?;
        let dirs_to_create = [
            crate::config::NEXT_DIR_PATH,
            crate::config::PROJECTS_DIR_PATH,
            crate::config::WAVE_DIR_PATH,
        ];
        for dir_path in dirs_to_create {
            fs::create_dir_all(config.base_path.join(dir_path))?;
        }

        todo!()
    }
}
