mod image_parser;
mod xcb;

use crate::error;
use image::RgbImage;
use x11rb::connection::Connection;
pub use xcb::get_display_info;

pub fn set_bg(image: RgbImage) -> error::Result<()> {
    let (conn, screen_num) = x11rb::connect(None)?;
    let screen = &conn.setup().roots[screen_num];
    let native_image = image_parser::parse_rgb_image(image, screen);
    xcb::set_bg_native(&conn, screen, native_image)
}
