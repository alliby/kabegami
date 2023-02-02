use crate::linux::desktop_env::DesktopEnv;
use std::{fs, env};
use std::path::{Path, PathBuf};
use std::process::Command;
use waraq::error::{Error, Result};
use waraq::image_utils::ImageMode;
use waraq::linux::xcb::get_display_info;

const GNOME_SETTER: &[u8] = include_bytes!("./scripts/gnome_setter.sh");
const KDE_SETTER: &[u8] = include_bytes!("./scripts/kde_setter.sh");
const XFCE_SETTER: &[u8] = include_bytes!("./scripts/xfce_setter.sh");
const LXDE_SETTER: &[u8] = include_bytes!("./scripts/lxde_setter.sh");
const LXQT_SETTER: &[u8] = include_bytes!("./scripts/lxqt_setter.sh");
const MATE_SETTER: &[u8] = include_bytes!("./scripts/mate_setter.sh");
const CINNAMON_SETTER: &[u8] = include_bytes!("./scripts/cinnamon_setter.sh");

fn config_dir() -> Result<PathBuf> {
    const HOME_KEY: &str = "HOME";
    if let Ok(p) = env::var(HOME_KEY) {
        let path: PathBuf = [&p, ".config", "kabegami"].iter().collect();
        Ok(path)
    } else {
        Err(Error::Other(format!("Environment variable {HOME_KEY} Not Found")))
    }
}

pub fn create_config_dir() -> Result<()> {
    let config_dir = config_dir()?;
    fs::create_dir_all(config_dir)?;
    Ok(())
}

pub fn copy_bg_with_mode(bg_path: PathBuf, mode: ImageMode) -> Result<PathBuf> {
    let config_bg = config_dir()?.join("current");
    let dim = get_display_info()?;
    mode.apply_with_save(&bg_path, &config_bg, dim)?;
    Ok(config_bg)
}

pub fn desktop_config_path(desktop: &DesktopEnv) -> Result<PathBuf> {
    let config_dir = config_dir()?;
    let path = match desktop {
        DesktopEnv::Gnome => config_dir.join("gnome_setter.sh"),
        DesktopEnv::Kde => config_dir.join("kde_setter.sh"),
        DesktopEnv::Xfce => config_dir.join("xfce_setter.sh"),
        DesktopEnv::Lxqt => config_dir.join("lxqt_setter.sh"),
        DesktopEnv::Lxde => config_dir.join("lxde_setter.sh"),
        DesktopEnv::Mate => config_dir.join("mate_setter.sh"),
        DesktopEnv::Cinnamon => config_dir.join("cinnamon_setter.sh"),
        DesktopEnv::Other => config_dir.join("setter.sh"),
    };
    Ok(path)
}

pub fn parse_default_setters<P>(path: P, desktop: &DesktopEnv) -> Result<()>
where
    P: AsRef<Path>,
{
    match desktop {
        DesktopEnv::Gnome => fs::write(path, GNOME_SETTER)?,
        DesktopEnv::Kde => fs::write(path, KDE_SETTER)?,
        DesktopEnv::Xfce => fs::write(path, XFCE_SETTER)?,
        DesktopEnv::Lxqt => fs::write(path, LXQT_SETTER)?,
        DesktopEnv::Lxde => fs::write(path, LXDE_SETTER)?,
        DesktopEnv::Mate => fs::write(path, MATE_SETTER)?,
        DesktopEnv::Cinnamon => fs::write(path, CINNAMON_SETTER)?,
        _ => (),
    };
    Ok(())
}

pub fn run_shell<P: AsRef<Path>>(shell_path: P, bg_path: P) -> Result<()> {
    let command_output = Command::new("sh")
        .arg(shell_path.as_ref())
        .arg(bg_path.as_ref())
        .output()?;
    if command_output.status.success() {
        Ok(())
    } else {
        let err_msg = String::from_utf8_lossy(&command_output.stderr);
        Err(Error::Other(format!("Failed to execute command:\n{err_msg}")))
    }
}
