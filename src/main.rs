mod cli;
mod options;

use clap::Parser;
use cli::{Cli, Commands};
use options::{CONFIG_PATH, OPTIONS, load_options};

fn main() {
    // Initialize Options
    let options = match load_options() {
        Ok(options) => options,
        Err(_) => panic!("could not open config file at {}", CONFIG_PATH),
    };
    OPTIONS.set(options).unwrap();

    let args = Cli::parse();
    match args.command {
        Commands::New(cmd) => cmd.execute(),
        Commands::Solve(cmd) => cmd.execute(),
    }
}
