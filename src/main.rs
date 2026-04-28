mod cli;

use aoc_solver::errors::AppErrors;
use aoc_solver::logs::init_logs;
use aoc_solver::options::{CONFIG_PATH, OPTIONS, load_options};
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<(), AppErrors> {
    // Initialize Options
    let options = match load_options() {
        Ok(options) => options,
        Err(e) => panic!("could not open config file at {}: {:?}", CONFIG_PATH, e),
    };
    OPTIONS.set(options).unwrap();

    match init_logs() {
        Ok(_) => {}
        Err(e) => return Err(e),
    }

    let args = Cli::parse();
    match args.command {
        Commands::New(cmd) => cmd.execute(),
        Commands::Solve(cmd) => cmd.execute(),
    }
}
