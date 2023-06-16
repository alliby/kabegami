mod cli;
pub mod image_utils;
#[cfg(target_os = "linux")]
mod linux;

use crate::image_utils::ImageMode;
use anyhow::anyhow;
use rand::prelude::IteratorRandom;
use rand::thread_rng;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

#[cfg(target_os = "linux")]
use linux::LinuxEnv as PlatformBackground;

// Check if a file is a valid image file
fn check_for_type<P: AsRef<Path>>(path: P) -> anyhow::Result<bool> {
    let mut f = std::fs::File::open(path)?;
    let mut buff = [0; 4];
    f.read_exact(&mut buff)?;
    Ok(infer::is_image(&buff))
}

/// A trait for setting wallpapers on different platforms
pub trait Platform {
    /// Set a specified wallpaper to the specified mode
    fn set_bg(path: PathBuf, mode: ImageMode) -> anyhow::Result<()>;

    /// sets a random wallpaper from a list of paths to the specified mode.
    /// filters the list to contain only valid image files, and calls the set_bg method.
    fn set_random_bg(
        paths_list: impl IteratorRandom<Item = PathBuf>,
        mode: ImageMode,
    ) -> anyhow::Result<()> {
        let mut rng = thread_rng();
        let random_path = paths_list
            .filter(|p| matches!(check_for_type(p), Ok(true)))
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
