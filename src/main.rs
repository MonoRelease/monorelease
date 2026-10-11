use clap::Parser;
use config::{Config, ConfigError, Environment, File};
use monorelease::config::Config as MonoReleaseConfig;

use crate::cli::Cli;

pub mod cli;

pub fn load_config() -> Result<MonoReleaseConfig, ConfigError> {
    let builder = Config::builder()
        .add_source(File::with_name(".monorelease/config.yaml").required(true))
        .add_source(Environment::with_prefix("MONORELEASE").separator("__"));
    builder.build()?.try_deserialize()
}

fn main() {
    let args = cli::Args::parse();
    Cli::run(args);
}
