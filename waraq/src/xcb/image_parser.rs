use image::{Rgb, RgbImage};
use x11rb::image::{BitsPerPixel, Image, ImageOrder, ScanlinePad};
use x11rb::protocol::xproto::Screen;

// convert image to native X image format
#[inline]
pub fn parse_rgb_image(rgb_image: RgbImage, screen: &Screen) -> Image<'static> {
    debug_assert!(rgb_image.width() < screen.width_in_pixels as u32);
    debug_assert!(rgb_image.height() < screen.height_in_pixels as u32);

    let mut native_image = Image::allocate(
        screen.width_in_pixels,
        screen.height_in_pixels,
        ScanlinePad::Pad32,
        screen.root_depth,
        BitsPerPixel::B32,
        ImageOrder::LsbFirst,
    );
    rgb_image
        .enumerate_pixels()
        .for_each(|(x, y, Rgb::<u8>([r, g, b]))| {
            let rgb = ((*r as u32) << 16) | ((*g as u32) << 8) | (*b as u32);
            native_image.put_pixel(x as u16, y as u16, rgb);
        });
    native_image
}
