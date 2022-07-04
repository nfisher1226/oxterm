use {gtk::gdk, rgba_simple::RGBA};

pub type Color = RGBA<f32>;
pub type Palette = [gdk::RGBA; 16];

pub struct Colors {
    foreground: Color,
    background: Color,
    black: Color,
    red: Color,
    green: Color,
    brown: Color,
    blue: Color,
    magenta: Color,
    cyan: Color,
    light_grey: Color,
    dark_grey: Color,
    light_red: Color,
    light_green: Color,
    yellow: Color,
    light_blue: Color,
    light_magenta: Color,
    light_cyan: Color,
    white: Color,
}

impl From<&Colors> for Palette {
    fn from(colors: &Colors) -> Self {
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
