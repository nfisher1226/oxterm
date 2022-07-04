mod background;
mod cursor;
mod dynamic_title_style;
mod font;
mod palette;
mod scrollback;

pub use {
    background::{gradient::Stop, Background, Gradient},
    cursor::Cursor,
    dynamic_title_style::DynamicTitleStyle,
    font::Font,
    palette::{Color, Colors, Palette},
    scrollback::Scrollback,
};

use {
    gtk::glib,
    std::{fs, path::PathBuf},
};
/// Returns an OS appropriate configuration directory path
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

/// Returns the path to config.toml
#[allow(clippy::must_use_candidate)]
pub fn get_config_file() -> PathBuf {
    let mut file = get_config_dir();
    file.push("config.toml");
    file
}

pub struct Config {}
