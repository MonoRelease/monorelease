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
    #[allow(clippy::struct_field_names, unused)]
    config_version: u16,
    // base_path: String,
    // #[serde(default)]
    // log_level: LogLevel,
}
