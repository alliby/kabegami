pub mod default_command;
pub mod desktop_env;

use crate::error::Result;
use crate::Platform;
use default_command::{create_config_dir, desktop_config_path, parse_default_setters, run_shell};
use desktop_env::DesktopEnv;
use std::path::{Path, PathBuf};
use wall::xlib::Xlib;

pub struct Linux {
    current_desktop: DesktopEnv,
    config_path: PathBuf
}

impl Linux {
    pub fn new() -> Result<Self> {
        create_config_dir()?;
        let current_desktop = DesktopEnv::get_current()?;
        let config_path: PathBuf = desktop_config_path(&current_desktop)?;
        Ok(Self {
            current_desktop,
            config_path
        })
    }

    pub fn xlib_set_bg<P: AsRef<Path>>(bg_path: P) -> Result<()> {
        let xlib = Xlib::new()?;
        Ok(xlib.set(bg_path, None)?)
    }

    pub fn shell_set_bg<P: AsRef<Path>>(&self, bg_path: P) -> Result<()> {
        if !self.config_path.exists() {
            parse_default_setters(&self.config_path, &self.current_desktop)?;
        };
        run_shell(self.config_path.as_path(), bg_path.as_ref())
    }

    pub fn set_bg<P: AsRef<Path>>(&self, bg_path: P) -> Result<()> {
        match (&self.config_path.exists(), &self.current_desktop) {
            (false, DesktopEnv::Other) => Self::xlib_set_bg(bg_path),
            _ => self.shell_set_bg(bg_path.as_ref()),
        }
    }
}

impl Platform for Linux {
    fn set_bg(bg_path: PathBuf) -> Result<()> {
        let linux = Linux::new()?;
        linux.set_bg(bg_path)
    }
}
