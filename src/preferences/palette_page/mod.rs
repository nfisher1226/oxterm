mod imp;

use {
    crate::config::{Colors, Palette},
    gtk::{
        glib::{self, Object},
        prelude::*,
        subclass::prelude::*,
    },
};

glib::wrapper! {
    pub struct PalettePage(ObjectSubclass<imp::PalettePage>)
        @extends gtk::Grid, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
            gtk::Orientable;
}

impl Default for PalettePage {
    fn default() -> Self {
        Self::new()
    }
}

impl PalettePage {
    #[must_use]
    pub fn new() -> Self {
        Object::new(&[]).expect("Cannot create palette page")
    }
}

impl From<&PalettePage> for Colors {
    fn from(page: &PalettePage) -> Self {
        let imp = page.imp();
        Self {
            black: imp.black_color.rgba().into(),
            red: imp.red_color.rgba().into(),
            green: imp.green_color.rgba().into(),
            brown: imp.brown_color.rgba().into(),
            blue: imp.blue_color.rgba().into(),
            magenta: imp.magenta_color.rgba().into(),
            cyan: imp.cyan_color.rgba().into(),
            light_grey: imp.light_grey_color.rgba().into(),
            dark_grey: imp.dark_grey_color.rgba().into(),
            light_red: imp.light_red_color.rgba().into(),
            light_green: imp.light_green_color.rgba().into(),
            yellow: imp.yellow_color.rgba().into(),
            light_blue: imp.light_blue_color.rgba().into(),
            light_magenta: imp.light_magenta_color.rgba().into(),
            light_cyan: imp.light_cyan_color.rgba().into(),
            white: imp.white_color.rgba().into(),
        }
    }
}

impl From<&PalettePage> for Palette {
    fn from(page: &PalettePage) -> Self {
        let imp = page.imp();
        [
            imp.black_color.rgba().into(),
            imp.red_color.rgba().into(),
            imp.green_color.rgba().into(),
            imp.brown_color.rgba().into(),
            imp.blue_color.rgba().into(),
            imp.magenta_color.rgba().into(),
            imp.cyan_color.rgba().into(),
            imp.light_grey_color.rgba().into(),
            imp.dark_grey_color.rgba().into(),
            imp.light_red_color.rgba().into(),
            imp.light_green_color.rgba().into(),
            imp.yellow_color.rgba().into(),
            imp.light_red_color.rgba().into(),
            imp.light_blue_color.rgba().into(),
            imp.light_cyan_color.rgba().into(),
            imp.white_color.rgba().into(),
        ]
    }
}
