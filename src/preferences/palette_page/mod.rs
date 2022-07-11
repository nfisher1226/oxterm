mod imp;

use {
    crate::{
        config::{ColorPalette, Palette},
        Values,
    },
    gtk::{
        glib::{self, GString, Object},
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

impl Values<ColorPalette> for PalettePage {
    fn values(&self) -> ColorPalette {
        let imp = self.imp();
        ColorPalette {
            name: imp
                .palette_selector
                .active_id()
                .unwrap_or(GString::from("Default"))
                .to_string(),
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

    fn set_values(&self, values: &ColorPalette) {
        let imp = self.imp();
        imp.palette_selector.set_active_id(Some(&values.name));
        imp.black_color.set_rgba(&values.black.into());
        imp.red_color.set_rgba(&values.red.into());
        imp.green_color.set_rgba(&values.green.into());
        imp.brown_color.set_rgba(&values.brown.into());
        imp.blue_color.set_rgba(&values.blue.into());
        imp.magenta_color.set_rgba(&values.magenta.into());
        imp.cyan_color.set_rgba(&values.cyan.into());
        imp.light_grey_color.set_rgba(&values.light_grey.into());
        imp.dark_grey_color.set_rgba(&values.dark_grey.into());
        imp.light_red_color.set_rgba(&values.light_red.into());
        imp.light_green_color.set_rgba(&values.light_green.into());
        imp.yellow_color.set_rgba(&values.yellow.into());
        imp.light_blue_color.set_rgba(&values.light_blue.into());
        imp.light_magenta_color
            .set_rgba(&values.light_magenta.into());
        imp.light_cyan_color.set_rgba(&values.light_cyan.into());
        imp.white_color.set_rgba(&values.white.into());
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
