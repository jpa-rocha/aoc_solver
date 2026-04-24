use crate::options::get_options;
use clap::Args;

#[derive(Args, Debug)]
pub struct SolveCmd {
    #[arg(short, long)]
    pub year: u16,

    #[arg(short, long)]
    pub day: u8,

    #[arg(short, long)]
    pub part: u8,
}

impl SolveCmd {
    pub fn execute(&self) {
        let year = self.year.to_string();
        let _ = get_options().solutions.path.clone().join(year);

        println!(
            "SOLVE: year {}, day {}, part {}",
            self.year, self.day, self.part
        )
    }
}
