pub mod error;
pub mod image_utils;
pub mod utils;

#[cfg(target_os = "linux")]
pub mod xcb;

#[cfg(target_os = "windows")]
pub mod win32;

use rand::seq::IteratorRandom;
use rand::thread_rng;
use std::io::Read;
use std::path::PathBuf;

#[derive(Debug, Default, Clone, Copy)]
pub enum PaperMode {
    /// This mode stretches and blur the image to fit the entire screen,
    /// and then add a scaled image on top of it
    #[default]
    Strim,
    /// This mode stretches the image to fit the entire screen, regardless of its aspect ratio.
    /// This is useful when the image is smaller than the screen resolution.
    Stretch,
    /// This mode fills the entire screen with the image, preserve the image ratio and cropping it if necessary.
    /// It is useful when you want to cover the entire screen with the image.
    Fill,
}

// Check if a file is a valid image file
fn is_image(path: &PathBuf) -> bool {
    let mut buff = [0; 4];
    std::fs::File::open(path)
        .and_then(|mut file| file.read_exact(&mut buff))
        .map(|_| infer::is_image(&buff))
        .unwrap_or(false)
}

/// A trait for setting wallpapers on different platforms
pub trait PaperSetter {
    /// Set a specified wallpaper to the specified mode
    fn set_wallpaper(path: PathBuf, mode: PaperMode) -> error::Result<()>;

    /// sets a random wallpaper from a list of paths to the specified mode.
    fn set_random_wallpaper(
        paths_list: impl IteratorRandom<Item = PathBuf>,
        mode: PaperMode,
    ) -> error::Result<()> {
        let mut rng = thread_rng();
        let random_path = paths_list
            .filter(is_image)
            .choose(&mut rng)
            .ok_or(error::PlatformError::InvalidDirectory)?;
        Self::set_wallpaper(random_path, mode)
    }
}
