#[derive(PartialEq, Debug)]
pub enum FolderCreationError {
    FolderAlreadyExists,
    CouldNotCreateFolder,
    CouldNotRead,
}
