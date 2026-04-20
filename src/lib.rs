use std::error::Error;
use std::path::PathBuf;

#[derive(PartialEq, Debug)]
pub enum FolderCreationError {
    FolderAlreadyExists,
    CouldNotCreateFolder,
}

pub fn create_folder(path: PathBuf) -> Result<(), FolderCreationError> {
    let exists = match path.try_exists() {
        Ok(x) => println!("this is x: {x}"),
        Err(x) => println!("this is errx :{x}"),
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_folder_success() {
        assert_eq!(create_folder(PathBuf::from("test_folder")), Ok(()));
    }

    #[test]
    fn test_create_folder_exists() {
        assert_eq!(
            create_folder(PathBuf::from("test_folder_exists")),
            Err(FolderCreationError::FolderAlreadyExists)
        )
    }
}
