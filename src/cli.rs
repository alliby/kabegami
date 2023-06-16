use crate::image_utils::ImageMode;
use argh::FromArgs;
use std::path::PathBuf;

fn modes_str_fn(_value: &str) -> Result<ImageMode, String> {
    match _value {
        "strim" => Ok(ImageMode::Strim),
        "fill" => Ok(ImageMode::Fill),
        "stretch" => Ok(ImageMode::Stretch),
        _ => Err(format!("no mode name \"{}\"", _value)),
    }
}

#[derive(FromArgs)]
/// Simple Background Setter
pub struct Cli {
    /// the path to the images directory or image file
    #[argh(positional, arg_name = "PATH")]
    pub path: PathBuf,

    /// default mode: strim, available modes: strim, stretch, fill
    #[argh(option, default = "ImageMode::default()", from_str_fn(modes_str_fn))]
    pub mode: ImageMode,
}
