mod image_modes;

use crate::PaperMode;
use image::error::ImageResult;
use image::ImageFormat;
use image::{DynamicImage, RgbImage};
use std::path::Path;

pub fn resize_image(image: DynamicImage, mode: PaperMode, dim: (u32, u32)) -> RgbImage {
    match mode {
        PaperMode::Strim => image_modes::strim_and_blur(image, dim),
        PaperMode::Fill => image_modes::fill(image, dim),
        PaperMode::Stretch => image_modes::stretch(image, dim),
    }
}

/// load the image from the input path, apply the mode and then save the modified image
/// to the dest path. It save it to Jpeg format by default
pub fn save_image<P: AsRef<Path>>(
    source: P,
    dest: P,
    mode: PaperMode,
    dim: (u32, u32),
) -> ImageResult<()> {
    let image = image::open(source)?;
    let img_out = resize_image(image, mode, dim);
    img_out.save_with_format(dest, ImageFormat::Jpeg)?;
    Ok(())
}
