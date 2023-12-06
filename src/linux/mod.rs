pub mod desktop_env;
pub mod utils;

use crate::{PaperMode, PaperSetter};
use anyhow::Result;
use desktop_env::DesktopEnv;
use std::path::PathBuf;
use utils::*;
use waraq::xcb::{self, get_display_info};

/// The Linux struct, used for setting the background on Linux systems.
pub struct LinuxEnv {
    /// The current desktop environment.
    current_desktop: DesktopEnv,
    /// The configuration path for the current desktop environment.
    config_path: PathBuf,
}

impl LinuxEnv {
    /// New instance of LinuxEnv with default values
    pub fn new() -> Result<Self> {
        let current_desktop = DesktopEnv::get_current()?;
        let config_path: PathBuf = desktop_config_path(&current_desktop)?;
        Ok(Self {
            current_desktop,
            config_path,
        })
    }

    /// Sets the background using shell commands.
    pub fn set_bg_shell(&self, bg_path: PathBuf, mode: PaperMode) -> Result<()> {
        let copied_path = copy_bg_with_mode(bg_path, mode)?;
        run_shell(&self.config_path, &copied_path)
    }

    /// Sets the background using XCB.
    pub fn set_bg_xcb(bg_path: PathBuf, mode: PaperMode) -> Result<()> {
        let image = image::open(bg_path)?;
        let dim = get_display_info()?;
        let resized_image = mode.apply(image, dim);
        Ok(xcb::set_bg(resized_image.as_raw())?)
    }
}

impl PaperSetter for LinuxEnv {
    /// Sets the background on a Linux system.
    /// This function check first if the shell script for the current desktop exists
    /// and execut it, It use XCB Library instead if the current desktop not supported
    /// and the `setter.sh` file not exists in the config dir
    fn set_bg(bg_path: PathBuf, mode: PaperMode) -> Result<()> {
        let env = Self::new()?;
        match (env.config_path.exists(), &env.current_desktop) {
            (false, DesktopEnv::Other) => Self::set_bg_xcb(bg_path, mode),
            (false, _) => {
                create_config_dir()?;
                parse_default_setters(&env.config_path, &env.current_desktop)?;
                env.set_bg_shell(bg_path, mode)
            }
            _ => env.set_bg_shell(bg_path, mode),
        }
    }
}
