use image::imageops::blur;
use image::{DynamicImage, RgbImage};
use image::{GenericImage, GenericImageView};
use resize::Pixel;
use resize::Type;
use rgb::FromSlice;

fn resize(image: &DynamicImage, dim: (usize, usize)) -> RgbImage {
    let (width, height) = (image.width() as usize, image.height() as usize);
    let src = image.to_rgb8();
    if dim == (width, height) {
        return src;
    }
    let mut resizer =
        resize::new(width, height, dim.0, dim.1, Pixel::RGB8, Type::Triangle).unwrap();
    let mut dst = RgbImage::new(dim.0 as u32, dim.1 as u32);
    resizer.resize(src.as_rgb(), dst.as_rgb_mut()).unwrap();
    dst
}

pub fn stretch(image: DynamicImage, dim: (u32, u32)) -> RgbImage {
    resize(&image, (dim.0 as usize, dim.1 as usize))
}

pub fn fill(image: DynamicImage, dim: (u32, u32)) -> RgbImage {
    let (w, h) = image.dimensions();
    let old_ratio = w as f32 / h as f32;
    let new_ratio = dim.0 as f32 / dim.1 as f32;
    if old_ratio == new_ratio {
        return stretch(image, dim);
    }
    // (w, h, x, y)
    let (new_w, new_h) = if old_ratio < new_ratio {
        (w, (w as f32 / new_ratio).round() as u32)
    } else {
        ((h as f32 * new_ratio).round() as u32, h)
    };
    let scaled_image = image.crop_imm((w - new_w) / 2, (h - new_h) / 2, new_w, new_h);
    resize(&scaled_image, (dim.0 as usize, dim.1 as usize))
}

pub fn strim_and_blur(image: DynamicImage, dim: (u32, u32)) -> RgbImage {
    let (w, h) = image.dimensions();
    let old_ratio = w as f32 / h as f32;
    let new_ratio = dim.0 as f32 / dim.1 as f32;
    if old_ratio == new_ratio {
        return stretch(image, dim);
    }
    // (w, h, x, y)
    let (new_w, new_h) = if old_ratio < new_ratio {
        ((dim.1 as f32 * old_ratio).round() as u32, dim.1)
    } else {
        (dim.0, (dim.0 as f32 / old_ratio).round() as u32)
    };
    let foreground = resize(&image, (new_w as usize, new_h as usize));
    let mut background = blur(&stretch(image, dim), 5.0);
    background
        .copy_from(&foreground, (dim.0 - new_w) / 2, (dim.1 - new_h) / 2)
        .unwrap();
    background
}
