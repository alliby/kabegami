use crate::error;
use std::path::Path;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use windows::Win32::UI::WindowsAndMessaging::*;

pub fn set_wallpaper<P: AsRef<Path>>(path: P) -> error::Result<()> {
    let wide: Vec<u16> = OsStr::new(path.as_ref())
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    unsafe {
	SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            Some(wide.as_ptr() as *mut _),
            SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
        )?;	
    }

    Ok(())
}
