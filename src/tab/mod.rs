mod imp;

use {
    crate::TabLabel,
    gtk::{
        glib::{self, Object},
        subclass::prelude::*,
    },
};

glib::wrapper! {
    pub struct Tab(ObjectSubclass<imp::Tab>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for Tab {
    fn default() -> Self {
        Self::new()
    }
}

impl Tab {
    pub fn new() -> Self {
        let name: String = std::iter::repeat_with(fastrand::alphanumeric)
            .take(10)
            .collect();
        Object::new(&[
            ("orientation", &gtk::Orientation::Horizontal),
            ("name", &name),
        ])
        .expect("Cannot create tab")
    }

    pub fn label(&self) -> TabLabel {
        self.imp().label.clone()
    }
}
