// Most of the code is from https://github.com/reujab/wallpaper.rs/blob/master/src/windows.rs
use crate::error::Result;
use std::io;
use std::path::Path;
use windows::Win32::Foundation::BOOL;
use windows::Win32::UI::WindowsAndMessaging::{
    SystemParametersInfoW, SPI_SETDESKWALLPAPER, SPIF_SENDCHANGE, SPIF_UPDATEINIFILE
};

pub fn set_bg(image_path: &Path) -> Result<()> {
    let path_ptr = image_path.to_string_lossy().as_ref().as_ptr();

    // Set the desktop wallpaper
    let result = unsafe {
        SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            Some(path_ptr as *mut _),
            SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
        )
    };
    if result != BOOL(0) {
        Ok(())
    } else {
        Err(io::Error::last_os_error().into())
    }
}
