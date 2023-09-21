/// Contain possible platform errors
mod error;

#[cfg(target_os = "linux")]
mod xcb;
#[cfg(target_os = "linux")]
pub use xcb::*;

#[cfg(target_os = "windows")]
mod windows_api;
#[cfg(target_os = "windows")]
pub use windows_api::*;
