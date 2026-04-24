use config::Config;
use serde::Deserialize;
use std::path::PathBuf;
use std::sync::OnceLock;

pub const CONFIG_PATH: &str = "/home/jrocha/projects/aoc_solver/config.toml";

pub static OPTIONS: OnceLock<Options> = OnceLock::new();

#[derive(Debug, Deserialize)]
pub struct Options {
    pub solutions: SolutionsOptions,
    pub input: InputOptions,
}

#[derive(Debug, Deserialize)]
pub struct SolutionsOptions {
    pub path: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct InputOptions {
    pub path: PathBuf,
}

pub fn load_options() -> Result<Options, config::ConfigError> {
    let settings = Config::builder()
        .add_source(config::File::with_name(CONFIG_PATH))
        .add_source(config::Environment::with_prefix("AOC"))
        .build()
        .unwrap();

    settings.try_deserialize()
}

pub fn get_options() -> &'static Options {
    OPTIONS.get().expect("options not initialized")
}
