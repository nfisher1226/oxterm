mod background;
mod cursor;
mod dynamic_title_style;
mod font;
mod general;
mod palette;
mod scrollback;
mod tab_position;
mod text;

pub use {
    background::{
        gradient::{
            Direction, HorizontalPlacement, Kind as GradientKind, Placement, Stop,
            VerticalPlacement,
        },
        image::{Image, Style as ImageStyle},
        Background, BackgroundColor, Gradient,
    },
    cursor::{Cursor, CursorStyle},
    dynamic_title_style::DynamicTitleStyle,
    font::Font,
    general::General,
    palette::{Color, ColorPalette, Palette},
    scrollback::Scrollback,
    tab_position::TabPosition,
    text::{Text, TextColor},
};

use {
    gtk::glib,
    std::{error::Error, fmt, fs, io, path::PathBuf},
};

/// Returns an OS appropriate configuration directory path (XDG_CONFIG_HOME)
///
/// # Panics
/// Can panic if the string returned from [`gtk::glib::user_config_dir`] is not valid
/// unicode (unlikely)
#[must_use]
pub fn get_config_dir() -> PathBuf {
    let mut configdir: PathBuf = glib::user_config_dir();
    let progname = env!("CARGO_PKG_NAME");
    configdir.push(progname);
    if !configdir.exists() {
        fs::create_dir(&configdir.to_str().unwrap()).unwrap_or_else(|e| eprintln!("{}", e));
    }
    configdir
}

/// Returns an OS appropriate data directory path (XDG_DATA_HOME)
///
/// # Panics
/// Can panic if the string returned from [`gtk::glib::user_config_dir`] is not valid
/// unicode (unlikely)
pub fn get_data_dir() -> PathBuf {
    let mut datadir = glib::user_data_dir();
    let progname = env!("CARGO_PKG_NAME");
    datadir.push(progname);
    if !datadir.exists() {
        fs::create_dir_all(&datadir.to_str().unwrap()).unwrap_or_else(|e| eprintln!("{}", e));
    }
    datadir
}

/// Returns the path to config.toml
#[allow(clippy::must_use_candidate)]
pub fn get_config_file() -> PathBuf {
    let mut file = get_config_dir();
    file.push("config.toml");
    file
}

#[derive(Debug)]
pub enum ConfigError {
    Io(io::Error),
    Format(ron::error::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "{e}"),
            Self::Format(e) => write!(f, "{e}"),
        }
    }
}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<ron::error::Error> for ConfigError {
    fn from(err: ron::error::Error) -> Self {
        Self::Format(err)
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            Self::Format(e) => Some(e),
        }
    }
}

pub struct Config {
    pub general: General,
    pub text: Text,
    pub background: Background,
}
