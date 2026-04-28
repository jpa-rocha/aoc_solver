pub mod errors;
pub mod logs;
pub mod options;

use errors::FolderCreationError;
use spdlog::{error, info, warn};
use std::{fs::create_dir, path::PathBuf};

pub fn create_folder(path: PathBuf) -> Result<bool, FolderCreationError> {
    let create_path = path.clone();

    let path_str = match path.to_str() {
        Some(r) => r,
        None => {
            warn!("could not convert provided path to str");
            "<NO PATH>"
        }
    };

    let exists = match create_path.try_exists() {
        Ok(x) => x,
        Err(_) => {
            error!("could not read folder: {}", path_str);
            return Err(FolderCreationError::CouldNotRead);
        }
    };

    if exists {
        info!("folder {} already exists", path_str);
        return Err(FolderCreationError::FolderAlreadyExists);
    } else {
        match create_dir(create_path) {
            Ok(()) => {
                info!("{} folder created successfuly", path_str);
                return Ok(true);
            }
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

        match create_folder(PathBuf::from(file_path)) {
            Ok(r) => {
                assert_eq!(r, true)
            }
            _ => {}
        }
    }

    #[test]
    fn test_create_folder_exists() {
        let tmp_dir = match TempDir::new("test_folder_exists") {
            Ok(x) => x,
            Err(_) => panic!("error at test_create_folder_success"),
        };

        let file_path = tmp_dir.path().join("test_folder_exists");

        // No PartialEq in FolderCreationError need to compare the strings
        match create_folder(file_path.clone()) {
            Err(e) => {
                assert_eq!(
                    e.to_string(),
                    FolderCreationError::FolderAlreadyExists.to_string()
                )
            }
            _ => {}
        }
    }
}
