mod cli;
pub mod image_utils;
#[cfg(target_os = "linux")]
mod linux;

use crate::image_utils::PaperMode;
use anyhow::anyhow;
use rand::prelude::IteratorRandom;
use rand::thread_rng;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

#[cfg(target_os = "linux")]
use linux::LinuxEnv as PlatformBackground;

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
    fn set_bg(path: PathBuf, mode: PaperMode) -> anyhow::Result<()>;

    /// sets a random wallpaper from a list of paths to the specified mode.
    /// filters the list to contain only valid image files, and calls the set_bg method.
    fn set_random_bg(
        paths_list: impl IteratorRandom<Item = PathBuf>,
        mode: PaperMode,
    ) -> anyhow::Result<()> {
        let mut rng = thread_rng();
        let random_path = paths_list
            .filter(|path| is_image(path))
            .choose(&mut rng)
            .ok_or(anyhow!("No valid image found !"));
        Self::set_bg(random_path?, mode)
    }
}

fn read_dir(path: PathBuf) -> anyhow::Result<impl Iterator<Item = PathBuf>> {
    Ok(path
        .read_dir()?
        .filter_map(|entry_result| entry_result.ok())
        .map(|dir_entry| dir_entry.path()))
}

fn main() -> anyhow::Result<()> {
    let cli: cli::Cli = argh::from_env();
    let path = cli.path;
    let mode = cli.mode;

    if path.is_dir() {
        let dir_paths = read_dir(path)?;
        PlatformBackground::set_random_bg(dir_paths, mode)?;
    } else {
        PlatformBackground::set_bg(path, mode)?;
    }
    Ok(())
}
