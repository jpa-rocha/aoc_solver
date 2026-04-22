pub mod cli;

use clap::Parser;
use cli::{Args, Commands::New};

fn main() {
    let args = Args::parse();
    match args.command {
        New { year, day, part } => println!("year {}, day {}, part {}", year, day, part),
    }
}
