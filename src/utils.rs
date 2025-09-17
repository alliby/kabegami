use crate::error;
use directories::BaseDirs;
use std::path::{Path, PathBuf};
use std::{fs, io};

use x11rb::connection::Connection;

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

pub fn read_dir(path: PathBuf) -> error::Result<impl Iterator<Item = PathBuf>> {
    Ok(path
        .read_dir()?
        .filter_map(|entry_result| entry_result.ok())
        .map(|dir_entry| dir_entry.path()))
}

// TODO : Add support for multiple monitor
#[cfg(target_os = "linux")]
pub fn screen_dimensions() -> error::Result<(u32, u32)> {
    use x11rb::connection::Connection;

    let (conn, screen_num) = x11rb::connect(None)?;
    let screen = &conn.setup().roots[screen_num];
    Ok((screen.width_in_pixels as _, screen.height_in_pixels as _))
}

#[cfg(target_os = "windows")]
pub fn screen_dimensions() -> error::Result<(u32, u32)> {
    use windows::Win32::Foundation::*;
    use windows::Win32::Graphics::Gdi::*;
    use windows::core::PWSTR;

    unsafe {
        let h_monitor = MonitorFromPoint(POINT { x: 0, y: 0 }, MONITOR_DEFAULTTOPRIMARY);
        let mut monitor_info: MONITORINFOEXW = std::mem::zeroed();
        monitor_info.monitorInfo.cbSize = size_of::<MONITORINFOEXW>() as u32;

        GetMonitorInfoW(h_monitor, &mut monitor_info.monitorInfo as *mut _).ok()?;

        let mut dev_mode: DEVMODEW = std::mem::zeroed();
        dev_mode.dmSize = size_of::<DEVMODEW>() as u16;

        EnumDisplaySettingsW(
            PWSTR(monitor_info.szDevice.as_mut_ptr()),
            ENUM_CURRENT_SETTINGS,
            &mut dev_mode as *mut _,
        ).ok()?;

        let width = dev_mode.dmPelsWidth;
        let height = dev_mode.dmPelsHeight;

        Ok((width, height))
    }
}
