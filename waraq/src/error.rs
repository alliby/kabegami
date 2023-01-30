use image::error::ImageError;

#[cfg(target_os = "linux")]
use x11rb::errors::{ConnectError, ConnectionError, ReplyOrIdError};

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(target_os = "linux")]
#[derive(Debug)]
pub enum X11Error {
    ConnectError(x11rb::errors::ConnectError),
    ReplyOrIdError(x11rb::errors::ReplyOrIdError),
}

#[cfg(target_os = "linux")]
impl std::fmt::Display for X11Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ConnectError(_err) => write!(f, "Failed to establish connection with X server"),
            Self::ReplyOrIdError(err) => write!(f, "{err}"),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    EnvError(String),
    CommandError(String),
    ImgError(image::error::ImageError),
    NoValidFile,
    #[cfg(target_os = "linux")]
    XcbError(X11Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::IoError(err) | Self::ImgError(ImageError::IoError(err)) => {
                write!(f, "Os error: {err}")
            }
            Self::EnvError(env) => write!(f, "Environment variable {env} Not Found"),
            Self::CommandError(cmd_err) => write!(f, "Failed to execute command:\n{cmd_err}"),
            Self::ImgError(img_err) => write!(f, "Failed to load image:\n{img_err}"),
            Self::NoValidFile => write!(f, "No valid file found"),
            #[cfg(target_os = "linux")]
            Self::XcbError(xcb_err) => write!(f, "Xcb Error:\n{xcb_err}"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<ImageError> for Error {
    fn from(err: ImageError) -> Self {
        Self::ImgError(err)
    }
}

#[cfg(target_os = "linux")]
impl From<ReplyOrIdError> for Error {
    fn from(err: ReplyOrIdError) -> Self {
        let reply_err = X11Error::ReplyOrIdError(err);
        Self::XcbError(reply_err)
    }
}

#[cfg(target_os = "linux")]
impl From<ConnectError> for Error {
    fn from(err: ConnectError) -> Self {
        let conn_err = X11Error::ConnectError(err);
        Self::XcbError(conn_err)
    }
}

#[cfg(target_os = "linux")]
impl From<ConnectionError> for Error {
    fn from(err: ConnectionError) -> Self {
        let conn_err = X11Error::ReplyOrIdError(ReplyOrIdError::ConnectionError(err));
        Self::XcbError(conn_err)
    }
}
