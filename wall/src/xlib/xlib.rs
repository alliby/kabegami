#![cfg(target_os = "linux")]

use {
    crate::xlib::{display_data::DisplayData, image_data::ImageData},
    anyhow::bail,
    image::ImageFormat,
    std::path::Path,
    x11::xlib::{
        Success, XCreateGC, XDestroyImage, XFlush, XFreeGC, XFreePixmap, XGCValues,
        XPutImage
    },
};

pub struct Xlib {
    displayd: DisplayData,
}

impl Xlib {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            displayd: DisplayData::new()?,
        })
    }

    /// Sets the image at `src_path` as the wallpaper.
    ///
    /// If format is `None` the image format will be guessed from the path and content of the file.
    pub fn set<P>(&self, src_path: P, format: Option<ImageFormat>) -> anyhow::Result<()>
    where
        P: AsRef<Path>,
    {
        let DisplayData {
            display,
            root_pixmap,
            ..
        } = self.displayd;

        let (ximage, width, height) = {
            let imaged = ImageData::new(src_path, format)?;
            let imaged = imaged.fill(&self.displayd);
            (
                imaged.to_ximage(&self.displayd)?,
                imaged.width,
                imaged.height,
            )
        };

        // Creates a Graphic Context for the root Pixmap.
        let gc = {
            let mut gc_init = default_xgc_values();
            unsafe { XCreateGC(display, root_pixmap, 0, &mut gc_init) }
        };
        if gc.is_null() {
            bail!("Failed to create an X Graphic Context for the root Pixmap");
        }
        // Draws the XImage to the root Pixmap.
        let result =
            unsafe { XPutImage(display, root_pixmap, gc, ximage, 0, 0, 0, 0, width, height) };
        if result != Success as i32 {
            bail!("Failed to put the XImage to the root Pixmap");
        }
        // Flushes the display.
        unsafe { XFlush(display) };

        unsafe { XFreePixmap(display, root_pixmap) };
        unsafe { XDestroyImage(ximage) };
        unsafe { XFreeGC(display, gc) };

        Ok(())
    }
}

fn default_xgc_values() -> XGCValues {
    XGCValues {
        foreground: Default::default(),
        background: Default::default(),
        function: Default::default(),
        plane_mask: Default::default(),
        line_width: Default::default(),
        line_style: Default::default(),
        cap_style: Default::default(),
        join_style: Default::default(),
        fill_style: Default::default(),
        fill_rule: Default::default(),
        arc_mode: Default::default(),
        tile: Default::default(),
        stipple: Default::default(),
        ts_x_origin: Default::default(),
        ts_y_origin: Default::default(),
        font: Default::default(),
        subwindow_mode: Default::default(),
        graphics_exposures: Default::default(),
        clip_x_origin: Default::default(),
        clip_y_origin: Default::default(),
        clip_mask: Default::default(),
        dash_offset: Default::default(),
        dashes: Default::default(),
    }
}
