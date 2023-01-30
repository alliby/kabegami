use argh::FromArgs;
use std::path::PathBuf;
use waraq::error;
use waraq::Platform;
use waraq::image_utils::ImageMode;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
use linux::LinuxEnv as PlatformBackground;

fn modes_str_fn(_value: &str) -> Result<ImageMode, String> {
    match _value {
        "strim" => Ok(ImageMode::Strim),
        "fill" => Ok(ImageMode::Fill),
        "stretch" => Ok(ImageMode::Stretch),
        _ => Err(format!("no mode name \"{_value}\""))
    }
}

#[derive(FromArgs)]
/// Simple Background Setter
struct Cli {
    /// the path to the images directory or image file
    #[argh(positional, arg_name = "PATH")]
    path: PathBuf,

    /// default mode: strim, Available modes: strim, stretch, fill
    #[argh(option, default = "ImageMode::default()", from_str_fn(modes_str_fn))]
    mode: ImageMode,
}

fn read_dir(path: PathBuf) -> error::Result<impl Iterator<Item=PathBuf>> {
    Ok(path.read_dir()?
        .filter_map(|entry_result| entry_result.ok())
        .map(|dir_entry| dir_entry.path())
    )
}

fn cli_run() -> error::Result<()> {
    let cli: Cli = argh::from_env();
    let path = cli.path;
    let mode = cli.mode;

    if path.is_dir() {
        let dir_paths = read_dir(path)?;
        PlatformBackground::set_random_bg(dir_paths, mode)?;
    } else {
        PlatformBackground::set_bg(path, mode)?;
    }
    Ok(())
    
}

fn main() {
    if let Err(e) = cli_run() {
        eprintln!("{e}")
    }
}
