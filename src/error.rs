pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Os Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Environment Variable {0} Not Found")]
    EnvError(String),
    #[error("Failed to execute command: \n{0}")]
    CommandError(String),
    #[error("No Valid File Found")]
    NoValidFile,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
