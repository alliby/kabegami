// See https://github.com/psychon/x11rb/issues/795

use crate::error::Result;
use std::borrow::Cow;
use x11rb::connection::Connection;
use x11rb::image::{BitsPerPixel, ColorComponent, Image, ImageOrder, PixelLayout, ScanlinePad};
use x11rb::protocol::xproto::*;

/// convert rgb pixel buffer to native X image format
#[inline]
pub fn parse_rgb_data<'a>(
    conn: &impl Connection,
    screen: &Screen,
    data: &'a [u8],
) -> Result<Image<'a>> {
    let native_image = Image::new(
        screen.width_in_pixels,
        screen.height_in_pixels,
        ScanlinePad::Pad8,
        screen.root_depth,
        BitsPerPixel::B24,
        ImageOrder::MsbFirst,
        Cow::Borrowed(data),
    )?;
    let native_image_layout = PixelLayout::new(
        ColorComponent::new(8, 16)?,
        ColorComponent::new(8, 8)?,
        ColorComponent::new(8, 0)?,
    );
    let pixel_layout = check_visual(screen, screen.root_visual);
    Ok(native_image
        .reencode(native_image_layout, pixel_layout, conn.setup())?
        .into_owned())
}

fn check_visual(screen: &Screen, id: Visualid) -> PixelLayout {
    // Find the information about the visual and at the same time check its depth.
    let visual_info = screen
        .allowed_depths
        .iter()
        .filter_map(|depth| {
            let info = depth.visuals.iter().find(|depth| depth.visual_id == id);
            info.map(|info| (depth.depth, info))
        })
        .next();
    let (depth, visual_type) = match visual_info {
        Some(info) => info,
        None => {
            eprintln!("Did not find the root visual's description?!");
            std::process::exit(1);
        }
    };
    // Check that the pixels have red/green/blue components that we can set directly.
    match visual_type.class {
        VisualClass::TRUE_COLOR | VisualClass::DIRECT_COLOR => {}
        _ => {
            eprintln!(
                "The root visual is not true / direct color, but {:?}",
                visual_type,
            );
            std::process::exit(1);
        }
    }
    let result = PixelLayout::from_visual_type(*visual_type)
        .expect("The server sent a malformed visual type");
    assert_eq!(result.depth(), depth);
    result
}
