use image::imageops::FilterType;
use image::DynamicImage;
use image::{GenericImage, GenericImageView};

pub fn fill(image: DynamicImage, dim: (u32, u32)) -> DynamicImage {
    image.resize_to_fill(dim.0, dim.1, FilterType::Triangle)
}

pub fn stretch(image: DynamicImage, dim: (u32, u32)) -> DynamicImage {
    image.resize_exact(dim.0, dim.1, FilterType::Triangle)
}

pub fn strim_and_blur(image: DynamicImage, dim: (u32, u32)) -> DynamicImage {
    let foreground = image.resize(dim.0, dim.1, FilterType::Triangle);
    let f_dim = foreground.dimensions();
    if f_dim == dim {
        return foreground;
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
    background
}
