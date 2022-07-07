mod imp;

use gtk::{
    glib::{self, clone, Object},
    prelude::*,
    subclass::prelude::*,
};

glib::wrapper! {
    pub struct BackgroundPage(ObjectSubclass<imp::BackgroundPage>)
        @extends gtk::Grid, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
            gtk::Orientable;
}

impl Default for BackgroundPage {
    fn default() -> Self {
        Self::new()
    }
}

impl BackgroundPage {
    #[must_use]
    pub fn new() -> Self {
        Object::new(&[]).expect("Cannot create background page")
    }
}
