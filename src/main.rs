use argh::FromArgs;
use kabegami::error;
use kabegami::{PaperMode, PaperSetter};
use std::path::PathBuf;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
use linux::LinuxSetter as PlatformSetter;

const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

fn modes_str_fn(_value: &str) -> Result<PaperMode, String> {
    match _value {
        "strim" => Ok(PaperMode::Strim),
        "fill" => Ok(PaperMode::Fill),
        "stretch" => Ok(PaperMode::Stretch),
        _ => Err(format!("no mode with name `{}`", _value)),
    }
}

#[derive(FromArgs)]
/// Simple Background Setter
pub struct Cli {
    /// the path to the images directory or image file
    #[argh(positional, arg_name = "PATH")]
    pub path: Option<PathBuf>,

    /// default mode: strim, available modes: strim, stretch, fill
    #[argh(option, default = "PaperMode::default()", from_str_fn(modes_str_fn))]
    pub mode: PaperMode,

    /// print version info
    #[argh(switch, short = 'V')]
    pub version: bool,
}

fn read_dir(path: PathBuf) -> error::Result<impl Iterator<Item = PathBuf>> {
    Ok(path
        .read_dir()?
        .filter_map(|entry_result| entry_result.ok())
        .map(|dir_entry| dir_entry.path()))
}

fn run() -> error::Result<()> {
    let cli: Cli = argh::from_env();
    if cli.version {
        println!("kabegami v{PKG_VERSION}");
        std::process::exit(0);
    }
    let Some(path) = cli.path else {
        eprintln!("no path was provided");
        std::process::exit(1);
    };
    let mode = cli.mode;
    
    if path.is_dir() {
        let dir_paths = read_dir(path)?;
        PlatformSetter::set_random_wallpaper(dir_paths, mode)
    } else {
        PlatformSetter::set_wallpaper(path, mode)
    }
}

fn main() {
    if let Err(err_msg) = run() {
        eprintln!("{err_msg}")
    }
}
