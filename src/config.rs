use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub enum LogLevel {
    #[default]
    Info,
    Warning,
    Error,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    log_level: LogLevel,
}
