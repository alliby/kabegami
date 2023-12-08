use kabegami::error;
use kabegami::image_utils;
use kabegami::PaperMode;
use image::Rgb;
use std::path::Path;
use x11rb::image::{BitsPerPixel, Image, ImageOrder, ScanlinePad};
use x11rb::protocol::xproto::Screen;

pub fn parse_file<P: AsRef<Path>>(
    wallpaper_path: P,
    screen: &Screen,
    mode: PaperMode,
) -> error::Result<Image<'static>> {
    let mut native_image = Image::allocate(
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
    let rgb_image = image::open(wallpaper_path)?;
    let resized_image = image_utils::resize_image(rgb_image, mode, (width, height));
    for (x, y, Rgb::<u8>([r, g, b])) in resized_image.enumerate_pixels() {
        let rgb = ((*r as u32) << 16) | ((*g as u32) << 8) | (*b as u32);
        native_image.put_pixel(x as u16, y as u16, rgb);
    }
    Ok(native_image)
}
