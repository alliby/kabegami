use rand::thread_rng;
use std::path::{Path, PathBuf};
use std::io::Read;
use rand::prelude::IteratorRandom;

#[cfg(target_os = "linux")]
pub mod linux;

pub mod error;

fn check_for_type<P: AsRef<Path>>(path: P) -> error::Result<bool> {
    let mut f = std::fs::File::open(path)?;
    let mut buff = [0; 4];
    f.read_exact(&mut buff)?;
    Ok(infer::is_image(&buff))
}

pub trait Platform {
    fn set_bg(path: PathBuf) -> error::Result<()>;

    fn set_random_bg(&self, list_path: impl IteratorRandom<Item=PathBuf>) -> error::Result<()> {
        let mut rng = thread_rng();
        let random_path = list_path
            .filter(|p| matches!(check_for_type(p), Ok(true)))
            .choose(&mut rng)
            .ok_or(error::Error::NoValidFile);
        Self::set_bg(random_path?)
    }
}
