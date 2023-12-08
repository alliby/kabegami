// Constants for different desktop environments
const GNOME_SESSIONS: [&str; 5] = ["pantheon", "gnome", "ubuntu", "deepin", "pop"];
const KDE_SESSIONS: [&str; 3] = ["neon", "plasma", "kde"];
const XFCE_SESSIONS: [&str; 2] = ["xfce", "xubuntu"];
const LXQT_SESSION: &str = "lxqt";
const LXDE_SESSION: &str = "lxde";
const MATE_SESSION: &str = "mate";
const CINNAMON_SESSION: &str = "cinnamon";
const DESKTOP_SESSION_KEYS: [&str; 2] = ["DESKTOP_SESSION", "XDG_CURRENT_DESKTOP"];

// Constants for desktop environments scripts
const GNOME_SETTER: &[u8] = include_bytes!("./scripts/gnome_setter.sh");
const KDE_SETTER: &[u8] = include_bytes!("./scripts/kde_setter.sh");
const XFCE_SETTER: &[u8] = include_bytes!("./scripts/xfce_setter.sh");
const LXDE_SETTER: &[u8] = include_bytes!("./scripts/lxde_setter.sh");
const LXQT_SETTER: &[u8] = include_bytes!("./scripts/lxqt_setter.sh");
const MATE_SETTER: &[u8] = include_bytes!("./scripts/mate_setter.sh");
const CINNAMON_SETTER: &[u8] = include_bytes!("./scripts/cinnamon_setter.sh");

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
            .find_map(|env_key| std::env::var(env_key).ok())
            .map(Self::from)
            .unwrap_or_default()
    }

    pub fn script_content(&self) -> Option<&[u8]> {
        match self {
            DesktopEnv::Gnome => Some(GNOME_SETTER),
            DesktopEnv::Kde => Some(KDE_SETTER),
            DesktopEnv::Xfce => Some(XFCE_SETTER),
            DesktopEnv::Lxqt => Some(LXQT_SETTER),
            DesktopEnv::Lxde => Some(LXDE_SETTER),
            DesktopEnv::Mate => Some(MATE_SETTER),
            DesktopEnv::Cinnamon => Some(CINNAMON_SETTER),
            _ => None,
        }
    }

    pub fn script_filename(&self) -> &str {
        match self {
            DesktopEnv::Gnome => "gnome_setter.sh",
            DesktopEnv::Kde => "kde_setter.sh",
            DesktopEnv::Xfce => "xfce_setter.sh",
            DesktopEnv::Lxqt => "lxqt_setter.sh",
            DesktopEnv::Lxde => "lxde_setter.sh",
            DesktopEnv::Mate => "mate_setter.sh",
            DesktopEnv::Cinnamon => "cinnamon_setter.sh",
            DesktopEnv::Other => "setter.sh",
        }
    }
}
