use crate::options::get_options;

use aoc_solver::create_folder;
use aoc_solver::errors::{AppErrors, FolderCreationError};

use clap::Args;

#[derive(Args, Debug)]
pub struct NewCmd {
    #[arg(short, long)]
    year: u16,
}

impl NewCmd {
    pub fn execute(&self) -> Result<(), AppErrors> {
        let year = self.year.to_string();

        let input_path = &get_options().input.path;

        // Create input folder if it doesnt exist
        match create_folder(input_path.clone()) {
            Ok(_) | Err(FolderCreationError::FolderAlreadyExists) => {}
            Err(e) => return Err(e.into()),
        };

        // Create year folder if it doesnt exist
        match create_folder(input_path.join(year)) {
            Ok(_) | Err(FolderCreationError::FolderAlreadyExists) => {}
            Err(e) => return Err(e.into()),
        };

        println!("NEW: year {}", self.year,);

        Ok(())
    }
}
