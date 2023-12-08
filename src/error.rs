use thiserror::Error;

/// Kabegami Main's Result type
pub type Result<T> = std::result::Result<T, PlatformError>;

/// Kabegami Main's Error Struct
#[derive(Error, Debug)]
pub enum PlatformError {
    #[cfg(target_os = "linux")]
    #[error("Failed to establish connection with X server")]
    ConnectError(#[from] x11rb::errors::ConnectError),

    #[cfg(target_os = "linux")]
    #[error("Failed to get Reply Id from X server")]
    ReplyOrIdError(#[from] x11rb::errors::ReplyOrIdError),

    #[cfg(target_os = "linux")]
    #[error("Error with X sever connection")]
    ConnectionError(#[from] x11rb::errors::ConnectionError),

    #[cfg(target_os = "linux")]
    #[error("Failed to send X server request")]
    ReplyError(#[from] x11rb::errors::ReplyError),

    #[cfg(target_os = "linux")]
    #[error("Error while parsing image")]
    ParseError(#[from] x11rb::rust_connection::ParseError),

    #[cfg(target_os = "windows")]
    #[error("{0}")]
    WindowsError(#[from] windows::core::Error),
    
    #[error("Image Error : {0}")]
    ImageError(#[from] image::error::ImageError),

    #[error("{0}")]
    IoError(#[from] std::io::Error)
}
