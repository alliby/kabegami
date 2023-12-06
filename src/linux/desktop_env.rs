use std::env;

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
#[derive(Debug, PartialEq, Eq, Default)]
pub enum DesktopEnv {
    Gnome,
    Kde,
    Xfce,
    Lxde,
    Lxqt,
    Mate,
    Cinnamon,
    #[default]
    Other,
}

impl<S: AsRef<str>> From<S> for DesktopEnv {
    fn from(session: S) -> Self {
        match session.as_ref() {
            s if GNOME_SESSIONS.iter().any(|gnome| s.contains(gnome)) => Self::Gnome,
            s if KDE_SESSIONS.iter().any(|kde| s.contains(kde)) => Self::Kde,
            s if XFCE_SESSIONS.iter().any(|xfce| s.contains(xfce)) => Self::Xfce,
            s if s.contains(LXQT_SESSION) => Self::Lxqt,
            s if s.contains(LXDE_SESSION) => Self::Lxde,
            s if s.contains(MATE_SESSION) => Self::Mate,
            s if s.contains(CINNAMON_SESSION) => Self::Cinnamon,
            _ => Self::Other,
        }
    }
}

impl DesktopEnv {
    /// get the current desktop environment by iterating over the environment variable keys
    /// Returning default `Desktop::Other` if no key has a value
    pub fn get_current() -> Self {
        DESKTOP_SESSION_KEYS
            .into_iter()
            .find_map(|env_key| env::var(env_key).ok())
            .map(Self::from)
            .unwrap_or_default()
    }
}
