use image::imageops::FilterType;
use image::{ DynamicImage, RgbImage};
use image::{GenericImage, GenericImageView};
use image::Pixel;

pub fn fill(image: DynamicImage, dim: (u32, u32)) -> RgbImage {
    let (input_width, input_height) = image.dimensions();
    let x_ratio = (input_width as f32) / (dim.0 as f32);
    let y_ratio = (input_height as f32) / (dim.1 as f32);

    RgbImage::from_fn(dim.0, dim.1, |x, y| {
        let x_index = (x as f32 * x_ratio) as u32;
        let y_index = (y as f32 * y_ratio) as u32;
        image.get_pixel(x_index, y_index).to_rgb()
    })
}

pub fn stretch(image: DynamicImage, dim: (u32, u32)) -> RgbImage {
    let (input_width, input_height) = image.dimensions();
    let x_ratio = (input_width - 1) as f32 / (dim.0 - 1) as f32;
    let y_ratio = (input_height - 1) as f32 / (dim.1 - 1) as f32;

    RgbImage::from_fn(dim.0, dim.1, |x, y| {
        let x_index = (x as f32 * x_ratio) as u32;
        let y_index = (y as f32 * y_ratio) as u32;
        image.get_pixel(x_index, y_index).to_rgb()
    })
}

pub fn strim_and_blur(image: DynamicImage, dim: (u32, u32)) -> RgbImage {
    let foreground = image.resize(dim.0, dim.1, FilterType::Nearest);
    let f_dim = foreground.dimensions();
    if f_dim == dim {
        return foreground.into_rgb8();
    }
    let mut background = image
        .resize_exact(dim.0, dim.1, FilterType::Nearest)
        .blur(5.0);
    if f_dim.0 == dim.0 {
        let height = dim.1;
        let new_h = f_dim.1;
        background
            .copy_from(&foreground, 0, (height - new_h) / 2)
            .unwrap();
    } else if f_dim.1 == dim.1 {
        let width = dim.0;
        let new_w = f_dim.0;
        background
            .copy_from(&foreground, (width - new_w) / 2, 0)
            .unwrap();
    }
    background.into_rgb8()
}

