use crate::config::Config;

pub mod config;
pub mod init;

#[derive(Debug, Clone)]
pub struct MonoRelease {
    config: Config,
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

impl MonoRelease {
    #[must_use]
    pub const fn new(config: Config) -> Self {
        Self { config }
    }

    #[must_use]
    pub fn version() -> String {
        VERSION.to_string()
    }
}
