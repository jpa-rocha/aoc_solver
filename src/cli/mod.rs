use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    New {
        #[arg(short, long)]
        year: u16,

        #[arg(short, long)]
        day: u8,

        #[arg(short, long)]
        part: u8,
    },
}
