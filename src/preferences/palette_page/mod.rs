mod imp;

use {
    crate::{config::Palette, Values},
    gtk::{
        glib::{self, clone, GString, Object},
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
        let obj: Self = Object::new(&[]).expect("Cannot create palette page");
        obj.imp().new_palette_name.connect_activate(
            clone!(@weak obj as palette_page => move |entry| {
                let mut new_palette = palette_page.values();
                new_palette.name = entry.text().to_string();
                if let Err(e) = new_palette.save() {
                    eprintln!("{e}");
                }
                palette_page
                    .imp()
                    .palette_selector
                    .append(Some(&new_palette.name.to_lowercase()), &new_palette.name);
                palette_page.set_values(&new_palette);
                palette_page.imp().new_popover.popdown();
            }),
        );
        let imp = obj.imp();
        for button in &[
            &imp.black_color,
            &imp.red_color,
            &imp.green_color,
            &imp.brown_color,
            &imp.blue_color,
            &imp.magenta_color,
            &imp.cyan_color,
            &imp.light_grey_color,
            &imp.dark_grey_color,
            &imp.light_red_color,
            &imp.light_green_color,
            &imp.yellow_color,
            &imp.light_blue_color,
            &imp.light_magenta_color,
            &imp.light_cyan_color,
            &imp.white_color,
        ] {
            button.connect_color_set(clone!(@weak obj as palette_page => move |_| {
                if let Err(e) = palette_page.values().save() {
                    eprintln!("{e}");
                }
            }));
        }
        obj
    }

    pub fn set_palette_list(&self) {
        for palette in &crate::config::palette::get_palette_names() {
            self.imp()
                .palette_selector
                .append(Some(&palette.0), &palette.1);
        }
    }
}

impl Values<Palette> for PalettePage {
    fn values(&self) -> Palette {
        let imp = self.imp();
        Palette {
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

    fn set_values(&self, values: &Palette) {
        let imp = self.imp();
        imp.palette_selector
            .set_active_id(Some(&values.name.to_lowercase()));
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
