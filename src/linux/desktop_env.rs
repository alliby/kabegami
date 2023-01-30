use std::env;
use waraq::error::{Error, Result};

// Constants for different desktop environments
const GNOME_SESSIONS: [&str; 5] = ["pantheon", "gnome", "ubuntu", "deepin", "pop"];
const KDE_SESSIONS: [&str; 3] = ["neon", "plasma", "kde"];
const XFCE_SESSIONS: [&str; 2] = ["xfce", "xubuntu"];
const LXQT_SESSION: &str = "lxqt";
const LXDE_SESSION: &str = "lxde";
const MATE_SESSION: &str = "mate";
const CINNAMON_SESSION: &str = "cinnamon";
const DESKTOP_SESSION_KEYS: [&str; 2] = ["DESKTOP_SESSION", "XDG_CURRENT_DESKTOP"];

/// Enum represent the different desktop environments supported
#[derive(Debug, PartialEq, Eq)]
pub enum DesktopEnv {
    Gnome,
    Kde,
    Xfce,
    Lxde,
    Lxqt,
    Mate,
    Cinnamon,
    Other,
}

impl From<&str> for DesktopEnv {
    fn from(session: &str) -> Self {
        if GNOME_SESSIONS.iter().any(|gnome| session.contains(gnome)) {
            Self::Gnome
        } else if KDE_SESSIONS.iter().any(|kde| session.contains(kde)) {
            Self::Kde
        } else if XFCE_SESSIONS.iter().any(|xfce| session.contains(xfce)) {
            Self::Xfce
        } else if session.contains(LXQT_SESSION) {
            Self::Lxqt
        } else if session.contains(LXDE_SESSION) {
            Self::Lxde
        } else if session.contains(MATE_SESSION) {
            Self::Mate
        } else if session.contains(CINNAMON_SESSION) {
            Self::Cinnamon
        } else {
            Self::Other
        }
    }
}

impl DesktopEnv {
    /// get the current desktop environment by iterating over the environment variable keys
    /// Returning an error if no key has a value
    pub fn get_current() -> Result<Self> {
        let desktop_session_values = DESKTOP_SESSION_KEYS.map(env::var);
        desktop_session_values
            .into_iter()
            .find(|env_result| env_result.is_ok())
            .map(|env_var| Self::from(env_var.unwrap().as_str()))
            .ok_or_else(|| Error::EnvError(DESKTOP_SESSION_KEYS.join(",")))
    }
}
