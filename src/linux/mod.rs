pub mod default_command;
pub mod desktop_env;

use default_command::*;
use desktop_env::DesktopEnv;
use std::path::PathBuf;

use waraq::error::Result;
use waraq::image_utils::ImageMode;
use waraq::linux::xcb;
use waraq::Platform;

/// The Linux struct, used for setting the background on Linux systems.
pub struct LinuxEnv {
    /// The current desktop environment.
    current_desktop: DesktopEnv,
    /// The configuration path for the current desktop environment.
    config_path: PathBuf,
}

impl LinuxEnv {
    pub fn new() -> Result<Self> {
        let current_desktop = DesktopEnv::get_current()?;
        let config_path: PathBuf = desktop_config_path(&current_desktop)?;
        Ok(Self {
            current_desktop,
            config_path,
        })
    }

    /// Sets the background using shell commands.
    /// TODO: add image modes
    pub fn set_bg_shell(&self, bg_path: PathBuf) -> Result<()> {
        parse_default_setters(&self.config_path, &self.current_desktop)?;
        run_shell(&self.config_path, &bg_path)
    }
    
    /// Sets the background using the XCB library.
    pub fn set_bg_xcb(bg_path: PathBuf, mode: ImageMode) -> Result<()> {
        xcb::set_bg(bg_path, mode)
    }
}

impl Platform for LinuxEnv {
    /// Sets the background on a Linux system.
    /// This function check first if the shell script for the current desktop exists 
    /// and execut it , It use XCB Library instead if the current desktop not supported
    /// and the `setter.sh` file not exists in the config dir
    fn set_bg(bg_path: PathBuf, mode: ImageMode) -> Result<()> {
        let env = Self::new()?;
        match (env.config_path.exists(), &env.current_desktop) {
            (false, DesktopEnv::Other) => Self::set_bg_xcb(bg_path, mode),
            _ => {
                create_config_dir()?;
                env.set_bg_shell(bg_path)
            }
        }
    }

}
