mod imp;

use gtk::{
    glib::{self, clone, Object},
    prelude::*,
    subclass::prelude::*,
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
}
