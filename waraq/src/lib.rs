/// Contain possible platform errors
mod error;

#[cfg(target_os = "linux")]
pub mod xcb;

#[cfg(target_os = "windows")]
pub mod windows_api;
