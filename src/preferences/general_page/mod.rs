mod imp;

use gtk::{
    glib::{self, clone, Object},
    prelude::*,
    subclass::prelude::*,
};

glib::wrapper! {
    pub struct GeneralPage(ObjectSubclass<imp::GeneralPage>)
        @extends gtk::Grid, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
            gtk::Orientable;
}

impl Default for GeneralPage {
    fn default() -> Self {
        Self::new()
    }
}

impl GeneralPage {
    #[must_use]
    pub fn new() -> Self {
        Object::new(&[]).expect("Cannot create general page")
    }
}
