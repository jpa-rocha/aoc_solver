use std::{fs::create_dir, path::PathBuf};

#[derive(PartialEq, Debug)]
pub enum FolderCreationError {
    FolderAlreadyExists,
    CouldNotCreateFolder,
    CouldNotRead,
}

pub fn create_folder(path: PathBuf) -> Result<bool, FolderCreationError> {
    let exists = match path.try_exists() {
        Ok(x) => x,
        Err(_) => return Err(FolderCreationError::CouldNotRead),
    };

    if exists {
        return Err(FolderCreationError::FolderAlreadyExists);
    } else {
        match create_dir(path) {
            Ok(()) => return Ok(true),
            Err(_) => return Err(FolderCreationError::CouldNotCreateFolder),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;

    #[test]
    fn test_create_folder_success() {
        let tmp_dir = match tempdir::TempDir::new("test_folder") {
            Ok(x) => x,
            Err(_) => panic!("error at test_create_folder_success"),
        };

        let file_path = tmp_dir.path().join("test_folder");

        assert_eq!(create_folder(PathBuf::from(file_path)), Ok(true));
    }

    #[test]
    fn test_create_folder_exists() {
        let tmp_dir = match TempDir::new("test_folder_exists") {
            Ok(x) => x,
            Err(_) => panic!("error at test_create_folder_success"),
        };

        let file_path = tmp_dir.path().join("test_folder_exists");

        let _ = create_folder(file_path.clone());
        assert_eq!(
            create_folder(file_path),
            Err(FolderCreationError::FolderAlreadyExists)
        )
    }
}
