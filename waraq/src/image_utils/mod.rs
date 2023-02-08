mod image_modes;
pub mod image_parser;

use image::{DynamicImage, RgbImage};
use image::ImageFormat;
use crate::error::Result;
use std::path::Path;

#[derive(Debug)]
pub enum ImageMode {
    /// This mode stretches and blur the image to fit the entire screen,
    /// and then add a scaled image on top of it
    Strim,
    /// This mode stretches the image to fit the entire screen, regardless of its aspect ratio.
    /// This is useful when the image is smaller than the screen resolution.
    Stretch,
    /// This mode fills the entire screen with the image, preserve the image ratio and cropping it if necessary.
    /// It is useful when you want to cover the entire screen with the image.
    Fill,
}

impl Default for ImageMode {
    fn default() -> Self {
        Self::Strim
    }
}

impl ImageMode {
    /// apply the mode to the given Image, it will return another modified image has the same dimentions
    pub fn apply(&self, image: DynamicImage, dim: (u32, u32)) -> RgbImage {
        match self {
            Self::Strim => image_modes::strim_and_blur(image, dim),
            Self::Fill => image_modes::fill(image, dim),
            Self::Stretch => image_modes::stretch(image, dim),
        }
    }

    /// load the image from the input path, apply the mode and then save the modified image
    /// to the dest path. It save it to Jpeg format by default
    pub fn apply_with_save(&self, input: &Path, dest: &Path, dim: (u32, u32)) -> Result<()> {
        let image = image::open(input)?;
        let img_out = self.apply(image, dim);
        img_out.save_with_format(dest, ImageFormat::Jpeg)?;
        Ok(())
    }
}
