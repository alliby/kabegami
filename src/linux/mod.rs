mod desktop_env;

use crate::utils;
use desktop_env::DesktopEnv;
use kabegami::error::Result;
use kabegami::{image_utils, xcb};
use kabegami::{PaperMode, PaperSetter};
use std::path::{Path, PathBuf};
use std::process::Command;

const DEFAULT_WALLPAPER_NAME: &str = "current";

pub fn run_script<P: AsRef<Path>>(script_path: P, wallpaper_path: P) -> Result<()> {
    let command_status = Command::new("sh")
        .arg(script_path.as_ref())
        .arg(wallpaper_path.as_ref())
        .status()?;
    match command_status.code() {
        Some(code) => println!("Exited with status code: {code}"),
        None => println!("Process terminated by signal"),
    }
    Ok(())
}

/// The Linux struct, used for setting the background on Linux systems.
pub struct LinuxSetter {
    /// The current desktop environment.
    current_desktop: DesktopEnv,
    /// The wallpaper path
    wallpaper_path: PathBuf,
    /// The wallpaper mode
    wallpaper_mode: PaperMode,
    /// The config path
    config_path: PathBuf,
}

impl LinuxSetter {
    /// New instance of LinuxSetter with default values
    pub fn new(wallpaper_path: PathBuf, wallpaper_mode: PaperMode) -> Self {
        Self {
            current_desktop: DesktopEnv::get_current(),
            wallpaper_path,
            wallpaper_mode,
            config_path: utils::config_dir(),
        }
    }

    /// Sets the background using shell commands.
    pub fn set_with_script(&self, script_path: PathBuf) -> Result<()> {
        let resized_image_path = self.config_path.join(DEFAULT_WALLPAPER_NAME);
        let screen_dimensions = utils::screen_dimensions()?;
        if !script_path.exists() {
            utils::create_dir(&self.config_path)?;
            if let Some(script_content) = self.current_desktop.script_content() {
                std::fs::write(&script_path, script_content)?;
            }
        }
        image_utils::save_image(
            &self.wallpaper_path,
            &resized_image_path,
            self.wallpaper_mode,
            screen_dimensions,
        )?;
        run_script(script_path, resized_image_path)
    }

    /// Sets with XCB
    pub fn set_with_xcb(&self) -> Result<()> {
        xcb::set_wallpaper(&self.wallpaper_path, self.wallpaper_mode)
    }
}

impl PaperSetter for LinuxSetter {
    /// Sets the background on a Linux system.
    /// This function check first if the shell script for the current desktop exists
    /// and execut it, It use XCB Library instead if the current desktop not supported
    /// and the `setter.sh` file not exists in the config dir
    fn set_wallpaper(wallpaper_path: PathBuf, mode: PaperMode) -> Result<()> {
        let setter = Self::new(wallpaper_path, mode);
        let script_path = setter
            .config_path
            .join(setter.current_desktop.script_filename());
        match (script_path.exists(), &setter.current_desktop) {
            (false, DesktopEnv::Other) => setter.set_with_xcb(),
            _ => setter.set_with_script(script_path),
        }
    }
}
