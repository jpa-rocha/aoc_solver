use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppErrors {
    #[error(transparent)]
    FolderCreation(#[from] FolderCreationError),

    #[error(transparent)]
    OpenFolder(#[from] OpenFolderError),

    #[error(transparent)]
    LoggerError(#[from] LoggerError),
}

#[derive(Error, Debug)]
pub enum FolderCreationError {
    #[error("Folder already exists")]
    FolderAlreadyExists,

    #[error("Could not create folder")]
    CouldNotCreateFolder,

    #[error("Could not read folder")]
    CouldNotRead,
}

#[derive(Error, Debug)]
pub enum OpenFolderError {
    #[error("Could not open folder")]
    CouldNotOpenFolder,
}

#[derive(Error, Debug)]
pub enum LoggerError {
    #[error("Could not initialize logger: {0}")]
    CouldNotInitializeLogger(#[from] spdlog::Error),
}
