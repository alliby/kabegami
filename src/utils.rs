use directories::BaseDirs;
use std::path::{Path, PathBuf};
use std::{fs, io};

const CONFIG_DIR_NAME: &str = "kabegami";

pub fn config_dir() -> PathBuf {
    BaseDirs::new()
        .map(|base_dir| base_dir.config_dir().join(CONFIG_DIR_NAME))
        .unwrap_or_default()
}

pub fn create_dir<P: AsRef<Path>>(dir_path: P) -> io::Result<()> {
    if dir_path.as_ref().exists() {
        return Ok(());
    }
    fs::create_dir_all(dir_path)
}

// TODO : Add support for multiple monitor
#![cfg(target_os = "linux")]
pub fn screen_dimensions() -> Result<(u32, u32)> {
    let (conn, screen_num) = x11rb::connect(None)?;
    let screen = &conn.setup().roots[screen_num];
    Ok((screen.width_in_pixels as _, screen.height_in_pixels as _))
}

