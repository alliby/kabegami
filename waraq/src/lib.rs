pub mod error;
pub mod image_utils;

use rand::prelude::IteratorRandom;
use rand::thread_rng;
use image_utils::ImageMode;
use std::io::Read;
use std::path::{Path, PathBuf};

#[cfg(target_os = "linux")]
pub mod linux;

fn check_for_type<P: AsRef<Path>>(path: P) -> error::Result<bool> {
    let mut f = std::fs::File::open(path)?;
    let mut buff = [0; 4];
    f.read_exact(&mut buff)?;
    Ok(infer::is_image(&buff))
}

pub trait Platform {
    fn set_bg(path: PathBuf, mode: ImageMode) -> error::Result<()>;

    fn set_random_bg(paths_list: impl IteratorRandom<Item = PathBuf>, mode: ImageMode) -> error::Result<()> {
        let mut rng = thread_rng();
        let random_path = paths_list
            .filter(|p| matches!(check_for_type(p), Ok(true)))
            .choose(&mut rng)
            .ok_or(error::Error::NoValidFile);
        Self::set_bg(random_path?, mode)
    }
}
