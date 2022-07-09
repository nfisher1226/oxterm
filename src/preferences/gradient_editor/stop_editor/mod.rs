mod imp;

use {
    crate::{
        config::{Color, Stop},
        Values,
    },
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
        let name: String = format!(
            "stop_{}",
            std::iter::repeat_with(fastrand::alphanumeric)
                .take(8)
                .collect::<String>()
        );
        let obj = Object::new(&[
            ("orientation", &gtk::Orientation::Horizontal),
            ("name", &name),
        ])
        .expect("Cannot create tab label");
        obj
    }

    pub fn new_with_stop(stop: &Stop) -> Self {
        let ed = Self::new();
        ed.set_values(stop);
        ed
    }

    #[must_use]
    pub fn button(&self) -> gtk::ColorButton {
        self.imp().button.clone()
    }

    pub fn set_color(&self, color: Color) {
        self.imp().button.set_rgba(&color.into());
    }

    #[must_use]
    pub fn scale(&self) -> gtk::SpinButton {
        self.imp().scale.clone()
    }

    pub fn set_position(&self, position: f64) {
        self.imp().scale.set_value(position);
    }
}

impl Values<Stop> for StopEditor {
    fn values(&self) -> Stop {
        Stop {
            color: self.button().rgba().into(),
            position: self.scale().value(),
        }
    }

    fn set_values(&self, stop: &Stop) {
        self.set_color(stop.color);
        self.set_position(stop.position);
    }
}
