use crate::utils;
use kabegami::error::Result;
use kabegami::{image_utils, win32};
use kabegami::{PaperMode, PaperSetter};
use std::path::PathBuf;

pub struct WindowsSetter {
    wallpaper_path: PathBuf,
    wallpaper_mode: PaperMode,
}

impl WindowsSetter {
    pub fn new(wallpaper_path: PathBuf, wallpaper_mode: PaperMode) -> Self {
        Self {
            wallpaper_path,
            wallpaper_mode,
        }
    }

    pub fn set_win32_wallpaper(&self) -> Result<()> {
        let config_path = utils::config_dir();
        let resized_image_path = config_path.join("current");
        utils::create_dir(config_path)?;
        let screen_dimensions = utils::screen_dimensions()?;
        image_utils::save_image(
            &self.wallpaper_path,
            &resized_image_path,
            self.wallpaper_mode,
            screen_dimensions,
        )?;
        win32::set_wallpaper(resized_image_path)
    }
}

impl PaperSetter for WindowsSetter {
    fn set_wallpaper(wallpaper_path: PathBuf, mode: PaperMode) -> Result<()> {
        let setter = Self::new(wallpaper_path, mode);
        setter.set_win32_wallpaper()
    }
}
