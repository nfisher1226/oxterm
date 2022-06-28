mod imp;

use gtk::{
    glib::{self, Object, ObjectExt},
    subclass::prelude::*,
};

glib::wrapper! {
    pub struct TabLabel(ObjectSubclass<imp::TabLabel>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Buildable;
}

impl Default for TabLabel {
    fn default() -> Self {
        Self::new()
    }
}

impl TabLabel {
    #[must_use]
    pub fn new() -> Self {
        Object::new(&[("orientation", &gtk::Orientation::Horizontal)])
            .expect("Cannot create tab label")
    }

    #[must_use]
    pub fn close_button(&self) -> gtk::Button {
        self.imp().button.clone()
    }

    pub fn set_label(&self, label: &str) {
        self.imp().label.set_label(label);
    }

    /// Connect to the "close-clicked" signal of the internal close button
    /// # Panics
    /// Panics if unable to get the object from the emitted signal (impossible)
    pub fn connect_close_clicked<F: Fn(&Self) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_local("close-clicked", true, move |values| {
            let obj = values[0].get::<Self>().unwrap();
            f(&obj);
            None
        })
    }
}
