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
    let mut image = Image::allocate(
        screen.width_in_pixels,
        screen.height_in_pixels,
        ScanlinePad::Pad32,
        screen.root_depth,
        BitsPerPixel::B32,
        ImageOrder::LsbFirst,
    );
    let (width, height) = (
        screen.width_in_pixels as u32,
        screen.height_in_pixels as u32,
    );
    let input = image::open(file_name)?;

    let input = mode.apply(input, (width, height));
    parse_rgb(&mut image, input.into_rgb8());

    Ok(image)
}
