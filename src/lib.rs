pub mod utils;
pub mod image_utils;
pub mod error;

use rand::seq::IteratorRandom;
use rand::thread_rng;
use std::path::{Path, PathBuf};
use std::io::Read;

#[derive(Debug, Default)]
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
fn is_image<P: AsRef<Path>>(path: P) -> bool {
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
    /// filters the list to contain only valid image files, and calls the set_bg method.
    fn set_random_wallpaper(
        paths_list: impl IteratorRandom<Item = PathBuf>,
        mode: PaperMode,
    ) -> error::Result<()> {
        let mut rng = thread_rng();
        let random_path = paths_list
            .filter(|path| is_image(path))
            .choose(&mut rng)
            .unwrap();
        Self::set_wallpaper(random_path, mode)
    }
}