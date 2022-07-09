use {
    gtk::gdk,
    rgba_simple::{PrimaryColor, RGBA},
    serde::{Deserialize, Serialize},
};

pub type Color = RGBA<f32>;
pub type Palette = [gdk::RGBA; 16];

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ColorPalette {
    pub name: String,
    pub black: Color,
    pub red: Color,
    pub green: Color,
    pub brown: Color,
    pub blue: Color,
    pub magenta: Color,
    pub cyan: Color,
    pub light_grey: Color,
    pub dark_grey: Color,
    pub light_red: Color,
    pub light_green: Color,
    pub yellow: Color,
    pub light_blue: Color,
    pub light_magenta: Color,
    pub light_cyan: Color,
    pub white: Color,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            name: String::from("NewPalette"),
            black: PrimaryColor::Black.into(),
            red: RGBA::new(0.647, 0.114, 0.176, 1.0),
            green: RGBA::new(0.0, 0.666, 0.0, 1.0),
            brown: RGBA::new(0.388, 0.270, 0.173, 1.0),
            blue: RGBA::new(0.0, 0.0, 0.666, 1.0),
            magenta: RGBA::new(0.666, 0.0, 0.666, 1.0),
            cyan: RGBA::new(0.0, 0.666, 0.666, 1.0),
            light_grey: RGBA::new(0.666, 0.666, 0.666, 1.0),
            dark_grey: RGBA::new(0.333, 0.333, 0.333, 1.0),
            light_red: RGBA::new(1.0, 0.333, 0.333, 1.0),
            light_green: RGBA::new(0.333, 1.0, 0.333, 1.0),
            yellow: RGBA::new(1.0, 0.741, 0.0, 1.0),
            light_blue: RGBA::new(0.333, 0.333, 1.0, 1.0),
            light_magenta: RGBA::new(1.0, 0.333, 1.0, 1.0),
            light_cyan: RGBA::new(0.333, 1.0, 1.0, 1.0),
            white: PrimaryColor::White.into(),
        }
    }
}

impl From<&ColorPalette> for Palette {
    fn from(colors: &ColorPalette) -> Self {
        [
            colors.black.into(),
            colors.red.into(),
            colors.green.into(),
            colors.brown.into(),
            colors.blue.into(),
            colors.magenta.into(),
            colors.cyan.into(),
            colors.light_grey.into(),
            colors.dark_grey.into(),
            colors.light_red.into(),
            colors.light_green.into(),
            colors.yellow.into(),
            colors.light_blue.into(),
            colors.light_magenta.into(),
            colors.light_cyan.into(),
            colors.white.into(),
        ]
    }
}
