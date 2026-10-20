pub mod config;
pub mod init;

pub struct MonoRelease;

const VERSION: &str = env!("CARGO_PKG_VERSION");

impl MonoRelease {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }

    #[must_use]
    pub fn version() -> String {
        VERSION.to_string()
    }
}

impl Default for MonoRelease {
    fn default() -> Self {
        Self::new()
    }
}
