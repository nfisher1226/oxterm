use {gtk::gdk, rgba_simple::RGBA};

pub type Color = RGBA<f32>;
pub type Palette = [gdk::RGBA; 16];

pub struct Colors {
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
