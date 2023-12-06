use crate::image_utils::PaperMode;
use argh::FromArgs;
use std::path::PathBuf;

fn modes_str_fn(_value: &str) -> Result<PaperMode, String> {
    match _value {
        "strim" => Ok(PaperMode::Strim),
        "fill" => Ok(PaperMode::Fill),
        "stretch" => Ok(PaperMode::Stretch),
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
    #[argh(option, default = "PaperMode::default()", from_str_fn(modes_str_fn))]
    pub mode: PaperMode,
}
