//! # Examples
//! 
//! - Set a single wallpaper in fill mode in linux
//! ```rust
//! use waraq::linux::xcb
//!
//! let path = PathBuf::from("/path/to/wallpaper");
//! let mode = image_utils::ImageMode::Fill;
//! xcb::set_bg(path, mode).unwrap();
//! ```
//! - Set a random wallpaper in stretch mode
//! ```rust
//! use waraq::linux::xcb
//!
//! let paths = vec![
//!     PathBuf::from("/path/to/wallpaper1"),
//!     PathBuf::from("/path/to/wallpaper2"),
//!     PathBuf::from("/path/to/wallpaper3"),
//! ];
//! let mode = image_utils::ImageMode::Stretch;
//! xcb::set_random_bg(paths.into_iter(), mode).unwrap();
//! ```
/// Contain possible platform and image errors
pub mod error;
/// Image utilities module
pub mod image_utils;

use rand::prelude::IteratorRandom;
use rand::thread_rng;
use image_utils::ImageMode;
use std::io::Read;
use std::path::{Path, PathBuf};

/// provides functions to set the background image on an X Window System display.
#[cfg(target_os = "linux")]
pub mod linux;

// Check if a file is a valid image file
fn check_for_type<P: AsRef<Path>>(path: P) -> error::Result<bool> {
    let mut f = std::fs::File::open(path)?;
    let mut buff = [0; 4];
    f.read_exact(&mut buff)?;
    Ok(infer::is_image(&buff))
}

/// A trait for setting wallpapers on different platforms
pub trait Platform {
    /// Set a specified wallpaper to the specified mode
    fn set_bg(path: PathBuf, mode: ImageMode) -> error::Result<()>;

    /// sets a random wallpaper from a list of paths to the specified mode.
    /// filters the list to contain only valid image files, and calls the set_bg method.
    fn set_random_bg(paths_list: impl IteratorRandom<Item = PathBuf>, mode: ImageMode) -> error::Result<()> {
        let mut rng = thread_rng();
        let random_path = paths_list
            .filter(|p| matches!(check_for_type(p), Ok(true)))
            .choose(&mut rng)
            .ok_or(error::Error::NoValidFile);
        Self::set_bg(random_path?, mode)
    }
}
