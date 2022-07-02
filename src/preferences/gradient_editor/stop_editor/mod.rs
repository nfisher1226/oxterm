mod imp;

use {
    crate::config::{Color, Stop},
    gtk::{
        glib::{self, Object},
        prelude::*,
        subclass::prelude::*,
    },
};

glib::wrapper! {
    pub struct StopEditor(ObjectSubclass<imp::StopEditor>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Buildable;
}

impl Default for StopEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl StopEditor {
    #[must_use]
    pub fn new() -> Self {
        Object::new(&[("orientation", &gtk::Orientation::Horizontal)])
            .expect("Cannot create tab label")
    }

    #[must_use]
    pub fn button(&self) -> gtk::ColorButton {
        self.imp().button.clone()
    }

    fn set_color(&self, color: Color) {
        self.imp().button.set_rgba(&color.into());
    }

    #[must_use]
    pub fn scale(&self) -> gtk::SpinButton {
        self.imp().scale.clone()
    }

    fn set_position(&self, position: f64) {
        self.imp().scale.set_value(position);
    }

    pub fn set_stop(&self, stop: &Stop) {
        self.set_color(stop.color);
        self.set_position(stop.position);
    }
}

impl From<&StopEditor> for Stop {
    fn from(editor: &StopEditor) -> Self {
        Stop {
            color: editor.button().rgba().into(),
            position: editor.scale().value(),
        }
    }
}
