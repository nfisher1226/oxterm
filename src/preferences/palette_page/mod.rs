mod imp;

use gtk::{
    glib::{self, clone, Object},
    prelude::*,
    subclass::prelude::*,
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
