use std::error::Error;

#[derive(PartialEq, Debug)]
enum FolderCreationError {
    FolderAlreadyExists,
    CouldNotCreateFolder,
}

pub fn create_folder(date: &str, day: &str) -> Result<(), FolderCreationError> {
    Ok(())
}
