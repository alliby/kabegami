mod desktop_env;
pub mod xcb;

use kabegami::{utils, image_utils};
use kabegami::{PaperMode, PaperSetter};
use crate::error::Result;
use desktop_env::DesktopEnv;
use std::path::{Path, PathBuf};
use std::process::Command;

const DEFAULT_WALLPAPER_NAME: &str = "current";

pub fn run_script<P: AsRef<Path>>(script_path: P, wallpaper_path: P) -> Result<()> {
    let _command_output = Command::new("sh")
        .arg(script_path.as_ref())
        .arg(wallpaper_path.as_ref())
        .output()?;
    // if command_output.status.success() {
    //     Ok(())
    // } else {
    //     let err_msg = String::from_utf8_lossy(&command_output.stderr);
    //     Err(anyhow::anyhow!("Failed to execute command:\n{err_msg}"))
    // }
    Ok(())
}


/// The Linux struct, used for setting the background on Linux systems.
pub struct LinuxSetter {
    /// The current desktop environment.
    current_desktop: DesktopEnv,
    /// The configuration path for the current desktop environment.
    config_path: PathBuf,
}

impl LinuxSetter {
    /// New instance of LinuxSetter with default values
    pub fn new() -> Self {
        Self {
            current_desktop: DesktopEnv::get_current(),
            config_path: utils::config_dir(),
        }
    }

    /// Sets the background using shell commands.
    pub fn set_with_script(&self, wallpaper_path: PathBuf, mode: PaperMode) -> Result<()> {
        let resized_image_path = self.config_path.join(DEFAULT_WALLPAPER_NAME);
        let script_path = self.config_path.join(self.current_desktop.script_filename());
        let screen_dimensions = xcb::screen_dimensions()?;
        if !script_path.exists() {
            utils::create_dir(&self.config_path)?;
            if let Some(script_content) = self.current_desktop.script_content() {
                std::fs::write(&script_path, script_content)?;
            }
        }
        image_utils::save_image(&wallpaper_path, &resized_image_path, mode, screen_dimensions)?;
        run_script(script_path, resized_image_path)
    }
}

impl PaperSetter for LinuxSetter {
    /// Sets the background on a Linux system.
    /// This function check first if the shell script for the current desktop exists
    /// and execut it, It use XCB Library instead if the current desktop not supported
    /// and the `setter.sh` file not exists in the config dir
    fn set_wallpaper(wallpaper_path: PathBuf, mode: PaperMode) -> Result<()> {
        let env = Self::new();
        match (env.config_path.exists(), &env.current_desktop) {
            (false, DesktopEnv::Other) => xcb::set_wallpaper(wallpaper_path, mode),
            _ => env.set_with_script(wallpaper_path, mode),
        }
    }
}
