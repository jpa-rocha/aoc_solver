mod new;
mod solve;

use clap::{Args as ClapArgs, Parser, Subcommand};
use new::NewCmd;
use solve::SolveCmd;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(ClapArgs, Debug)]
pub struct Args {}

#[derive(Subcommand, Debug)]
pub enum Commands {
    New(NewCmd),
    Solve(SolveCmd),
}
