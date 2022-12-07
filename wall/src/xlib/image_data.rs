#![cfg(target_os = "linux")]

use {
    crate::xlib::display_data::DisplayData,
    image::{
        imageops::FilterType, io::Reader, DynamicImage, GenericImageView, ImageBuffer, Rgb,
        RgbImage,
    },
    std::{convert::TryFrom, path::Path},
    x11::xlib::{XAllPlanes, XGetImage, XGetPixel, XImage, XPutPixel, ZPixmap},
};

pub(crate) struct ImageData {
    pub(crate) image: RgbImage,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl ImageData {
    pub(crate) fn new<P>(src_path: P, format: Option<image::ImageFormat>) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        let im = {
            if let Some(format) = format {
                let mut im = Reader::open(src_path)?;
                im.set_format(format);
                im.decode()?
            } else {
                Reader::open(src_path)?.with_guessed_format()?.decode()?
            }
        };
        let (width, height) = im.dimensions();
        Ok(Self {
            width,
            height,
            image: im.into_rgb8(),
        })
    }

    pub(crate) fn fill(self, displayd: &DisplayData) -> Self {
        if self.width == displayd.width && self.height == displayd.height {
            return self;
        }
        let image = DynamicImage::ImageRgb8(self.image);
        let nimage = image.resize_to_fill(displayd.width, displayd.height, FilterType::Nearest);
        Self {
            image: nimage.to_rgb8(),
            width: displayd.width,
            height: displayd.height,
        }
    }
}

impl TryFrom<*mut XImage> for ImageData {
    type Error = anyhow::Error;

    fn try_from(img: *mut XImage) -> Result<Self, Self::Error> {
        let (width, height) = unsafe { ((*img).width as u32, (*img).height as u32) };

        let mut imbuf = ImageBuffer::new(width, height);
        for (x, y, pixel) in imbuf.enumerate_pixels_mut() {
            // TODO: Add support for RGBA.
            let xpixel = unsafe { XGetPixel(img, x as i32, y as i32) };
            let r = (xpixel >> 16) as u8;
            let g = ((xpixel & 0x00ff00) >> 8) as u8;
            let b = (xpixel & 0x0000ff) as u8;
            *pixel = Rgb([r, g, b]);
        }

        Ok(ImageData {
            image: imbuf,
            width: width as u32,
            height: height as u32,
        })
    }
}

impl ImageData {
    pub(crate) fn to_ximage(&self, displayd: &DisplayData) -> anyhow::Result<*mut XImage> {
        let ximage = unsafe {
            XGetImage(
                displayd.display,
                displayd.root_pixmap,
                0,
                0,
                self.width,
                self.height,
                XAllPlanes(),
                ZPixmap,
            )
        };

        for (x, y, Rgb::<u8>([r, g, b])) in self.image.enumerate_pixels() {
            let pixel = ((*r as u64) << 16) + ((*g as u64) << 8) + (*b as u64);
            unsafe { XPutPixel(ximage, x as i32, y as i32, pixel) };
        }

        Ok(ximage)
    }
}
