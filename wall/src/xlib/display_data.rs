#![cfg(target_os = "linux")]

use {
    anyhow::bail,
    std::{
        env,
        ffi::{c_void, CString},
        mem::MaybeUninit,
        os::raw::{c_int, c_uchar, c_uint, c_ulong},
    },
    x11::xlib::{
        Atom, Display, False, Pixmap, Success, XDefaultScreen, XFree, XFreePixmap, XGetGeometry,
        XGetWindowProperty, XInternAtom, XOpenDisplay, XRootWindow, XA_PIXMAP,
    },
};

pub(crate) struct DisplayData {
    /// Active display.
    pub(crate) display: *mut Display,
    /// ID of the root window.
    pub(crate) root_win: c_ulong,
    /// The pixmap of the root window.
    pub(crate) root_pixmap: Pixmap,
    pub(crate) width: u32,
    pub(crate) height: u32,
    #[allow(dead_code)]
    pub(crate) depth: u32,
}

impl DisplayData {
    pub(crate) fn new() -> anyhow::Result<Self> {
        let display = {
            let display_name = env::var("DISPLAY")?;
            let display_name = CString::new(display_name)?;
            let display = unsafe { XOpenDisplay(display_name.as_ptr()) };
            if display.is_null() {
                bail!(
                    "Cannot connect to X Server ({})",
                    display_name.into_string().unwrap()
                );
            }
            display
        };

        let root_win = {
            let default_screen = unsafe { XDefaultScreen(display) };
            unsafe { XRootWindow(display, default_screen) }
        };

        let (width, height, depth) = {
            let mut root_win_ = MaybeUninit::<c_ulong>::uninit();
            let mut x = MaybeUninit::<c_int>::uninit();
            let mut y = MaybeUninit::<c_int>::uninit();
            let mut width = MaybeUninit::<c_uint>::uninit();
            let mut height = MaybeUninit::<c_uint>::uninit();
            let mut border_width = MaybeUninit::<c_uint>::uninit();
            let mut depth = MaybeUninit::<c_uint>::uninit();
            let status = unsafe {
                XGetGeometry(
                    display,
                    root_win,
                    root_win_.as_mut_ptr(),
                    x.as_mut_ptr(),
                    y.as_mut_ptr(),
                    width.as_mut_ptr(),
                    height.as_mut_ptr(),
                    border_width.as_mut_ptr(),
                    depth.as_mut_ptr(),
                )
            };
            // This usually returns a non Success exit status even though it works, so we'll only
            // log instead of (bail!)ing.
            if status != Success as i32 {
                log::info!(
                    "Possible error while getting root window's geometry, XGetGeometry returned non Success status -> {}",
                    status
                );
            }
            let width = unsafe { width.assume_init() };
            let height = unsafe { height.assume_init() };
            let depth = unsafe { depth.assume_init() };
            (width, height, depth)
        };

        let root_pixmap = {
            let cstring = CString::new("_XROOTPMAP_ID").unwrap();
            let x_root_pmap_id = unsafe { XInternAtom(display, cstring.as_ptr(), False) };

            let mut act_type = MaybeUninit::<Atom>::uninit();
            let mut act_format = MaybeUninit::<c_int>::uninit();
            let mut nitems = MaybeUninit::<c_ulong>::uninit();
            let mut bytes_after = MaybeUninit::<c_ulong>::uninit();
            let mut data: *mut c_uchar = std::ptr::null_mut();

            let status = unsafe {
                XGetWindowProperty(
                    display,
                    root_win,
                    x_root_pmap_id,
                    0,
                    1,
                    False,
                    XA_PIXMAP,
                    act_type.as_mut_ptr(),
                    act_format.as_mut_ptr(),
                    nitems.as_mut_ptr(),
                    bytes_after.as_mut_ptr(),
                    &mut data,
                )
            };
            if status != Success as i32 {
                bail!(
                    "Failed to get root pixmap, XGetWindowProperty returned non Success status -> {}",
                    status
                );
            }
            if data.is_null() {
                bail!("Failed to get root pixmap, XGetWindowProperty returned NULL");
            } else {
                let root_pixmap = unsafe { *(data as *const Pixmap) };
                unsafe { XFree(data as *mut c_void) };
                root_pixmap
            }
        };

        Ok(Self {
            display,
            root_win,
            root_pixmap,
            width,
            height,
            depth,
        })
    }
}

impl Drop for DisplayData {
    fn drop(&mut self) {
        unsafe { XFreePixmap(self.display, self.root_pixmap) };
    }
}
