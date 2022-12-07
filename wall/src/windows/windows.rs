#![cfg(windows)]

use {
    std::{
        ffi::OsStr,
        io, iter,
        os::{raw::c_void, windows::ffi::OsStrExt},
        path::Path,
    },
    winapi::um::winuser::{
        SystemParametersInfoW, SPIF_SENDCHANGE, SPIF_UPDATEINIFILE,
        SPI_SETDESKWALLPAPER,
    },
};

/// Sets the wallpaper given the full path of an image.
pub fn set<P>(full_path: P) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    let path = OsStr::new(full_path.as_ref())
        .encode_wide()
        // Append null byte
        .chain(iter::once(0))
        .collect::<Vec<u16>>();

    let success = unsafe {
        SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            path.as_ptr() as *mut c_void,
            SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
        ) == 1
    };

    if success {
        Ok(())
    } else {
        Err(io::Error::last_os_error().into())
    }
}
