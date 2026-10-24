use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::error;
use tracing_subscriber::{
    EnvFilter, filter::LevelFilter, layer::SubscriberExt, reload, util::SubscriberInitExt,
};

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub enum LogLevel {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

pub struct Log {
    filter_reload_handle: reload::Handle<EnvFilter, tracing_subscriber::Registry>,
}

impl Log {
    #[must_use]
    pub fn init(log_level: &LogLevel) -> Self {
        let level_filter: LevelFilter = log_level.into();
        let filter = EnvFilter::builder()
            .with_default_directive(level_filter.into())
            .from_env_lossy();

        let (filter_layer, reload_handle) = reload::Layer::new(filter);

        let fmt_layer = tracing_subscriber::fmt::layer();

        tracing_subscriber::registry()
            .with(filter_layer)
            .with(fmt_layer)
            .init();

        Self {
            filter_reload_handle: reload_handle,
        }
    }

    pub fn set_log_level(&self, log_level: &LogLevel) -> Result<()> {
        let level_filter: LevelFilter = log_level.into();

        let new_filter = EnvFilter::builder()
            .with_default_directive(level_filter.into())
            .from_env_lossy();

        self.filter_reload_handle
            .modify(|filter| *filter = new_filter)
            .map_err(|err| {
                error!("failed to update log level: {err}");
                anyhow::Error::from(err)
            })
    }
}

impl From<&LogLevel> for LevelFilter {
    fn from(level: &LogLevel) -> Self {
        match level {
            LogLevel::Trace => Self::TRACE,
            LogLevel::Debug => Self::DEBUG,
            LogLevel::Info => Self::INFO,
            LogLevel::Warn => Self::WARN,
            LogLevel::Error => Self::ERROR,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use tracing::level_filters::LevelFilter;
    use tracing_subscriber::EnvFilter;

    pub fn init_test_logging() {
        let filter = EnvFilter::builder()
            .with_default_directive(LevelFilter::DEBUG.into())
            .from_env_lossy();

        let _ = tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_test_writer()
            .try_init();
    }
}
