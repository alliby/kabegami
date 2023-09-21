mod image_parser;
mod xcb;

use crate::error;
use x11rb::connection::Connection;
pub use xcb::get_screen_dimensions;

pub fn set_bg(image_data: &[u8]) -> error::Result<()> {
    let (conn, screen_num) = x11rb::connect(None)?;
    let screen = &conn.setup().roots[screen_num];
    let native_image = image_parser::parse_rgb_data(&conn, screen, image_data)?;
    xcb::set_bg_native(&conn, screen, native_image)
}
