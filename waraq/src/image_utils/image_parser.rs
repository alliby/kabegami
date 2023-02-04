use crate::error;
use crate::image_utils::ImageMode;

use image::{Rgb, RgbImage};
use std::ffi::OsStr;

#[cfg(target_os = "linux")]
use x11rb::image::{BitsPerPixel, Image, ImageOrder, ScanlinePad};
#[cfg(target_os = "linux")]
use x11rb::protocol::xproto::Screen;

#[cfg(target_os = "linux")]
fn parse_rgb(image: &mut Image, input: RgbImage) {
    input
        .enumerate_pixels()
        .for_each(|(x, y, Rgb::<u8>([r, g, b]) )| {
            let rgb = ((*r as u32) << 16) | ((*g as u32) << 8) | (*b as u32);
            image.put_pixel(x as u16, y as u16, rgb);
        })
}

#[cfg(target_os = "linux")]
pub fn parse_file(
    file_name: &OsStr,
    screen: &Screen,
    mode: ImageMode,
) -> error::Result<Image<'static>> {
    let (width, height) = (
        screen.width_in_pixels,
        screen.height_in_pixels,
    );

    let image = image::open(file_name)?;
    let new_image = mode.apply(image, (width as u32, height as u32));
    
    let mut ximage = Image::allocate(
        width,
        height,
        ScanlinePad::Pad32,
        screen.root_depth,
        BitsPerPixel::B32,
        ImageOrder::LsbFirst,
    );

    parse_rgb(&mut ximage, new_image.into_rgb8());

    Ok(ximage)
}
