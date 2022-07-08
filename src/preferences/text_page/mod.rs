mod imp;

use {
    crate::config::{Cursor, Font, Scrollback, Text, TextColor},
    gtk::{
        glib::{self, GString, Object},
        prelude::*,
        subclass::prelude::*,
    },
};

glib::wrapper! {
    pub struct TextPage(ObjectSubclass<imp::TextPage>)
        @extends gtk::Grid, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
            gtk::Orientable;
}

impl Default for TextPage {
    fn default() -> Self {
        Self::new()
    }
}

impl TextPage {
    #[must_use]
    pub fn new() -> Self {
        Object::new(&[]).expect("Cannot create text page")
    }

    pub fn cursor(&self) -> Cursor {
        let imp = self.imp();
        Cursor {
            style: imp
                .cursor_style
                .active_id()
                .unwrap_or(GString::from("Block"))
                .as_str()
                .into(),
            blinks: imp.cursor_blinks.is_active(),
        }
    }

    pub fn set_cursor(&self, cursor: &Cursor) {
        let imp = self.imp();
        imp.cursor_style
            .set_active_id(Some(&cursor.style.to_string().to_lowercase()));
        imp.cursor_blinks.set_active(cursor.blinks);
    }

    pub fn scrollback(&self) -> Scrollback {
        let imp = self.imp();
        if imp.infinite_scrollback.is_active() {
            Scrollback::Infinite
        } else {
            Scrollback::Finite(imp.scrollback_lines.value())
        }
    }

    pub fn set_scrollback(&self, scrollback: &Scrollback) {
        let imp = self.imp();
        match scrollback {
            Scrollback::Infinite => {
                imp.infinite_scrollback.set_active(true);
                imp.scrollback_lines.set_sensitive(false);
            }
            Scrollback::Finite(n) => {
                imp.infinite_scrollback.set_active(false);
                imp.scrollback_lines.set_sensitive(true);
                imp.scrollback_lines.set_value(*n);
            }
        }
    }

    pub fn font(&self) -> Font {
        let imp = self.imp();
        if imp.system_font.is_active() {
            Font::System
        } else {
            Font::Custom(imp.font_chooser_button.font_desc().unwrap().into())
        }
    }

    pub fn set_font(&self, font: &Font) {
        let imp = self.imp();
        match font {
            Font::System => {
                imp.system_font.set_active(true);
                imp.font_chooser_button.set_sensitive(false);
            }
            Font::Custom(f) => {
                imp.system_font.set_active(false);
                imp.font_chooser_button.set_sensitive(true);
                imp.font_chooser_button.set_font_desc(&f.into());
            }
        }
    }

    pub fn color(&self) -> TextColor {
        let imp = self.imp();
        match imp
            .color_type
            .active_id()
            .unwrap_or(GString::from("white"))
            .as_str()
        {
            "white" => TextColor::White,
            "black" => TextColor::Black,
            "custom" => TextColor::Custom(imp.text_color.rgba().into()),
            _ => TextColor::White,
        }
    }

    pub fn set_color(&self, color: &TextColor) {
        let imp = self.imp();
        match color {
            TextColor::White => {
                imp.text_color.set_sensitive(false);
                imp.color_type.set_active_id(Some("white"));
            }
            TextColor::Black => {
                imp.text_color.set_sensitive(false);
                imp.color_type.set_active_id(Some("black"));
            }
            TextColor::Custom(c) => {
                imp.text_color.set_sensitive(true);
                imp.color_type.set_active_id(Some("custom"));
                imp.text_color.set_rgba(&c.clone().into());
            }
        }
    }

    pub fn text(&self) -> Text {
        Text {
            cursor: self.cursor(),
            scrollback: self.scrollback(),
            font: self.font(),
            color: self.color(),
        }
    }

    pub fn set_text(&self, text: &Text) {
        self.set_cursor(&text.cursor);
        self.set_scrollback(&text.scrollback);
        self.set_font(&text.font);
        self.set_color(&text.color);
    }
}
