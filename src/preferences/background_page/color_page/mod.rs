mod imp;

use gtk::{
    glib::{self, clone, Object},
    prelude::*,
    subclass::prelude::*,
};

glib::wrapper! {
    pub struct ColorPage(ObjectSubclass<imp::ColorPage>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
            gtk::Orientable;
}

impl Default for ColorPage {
    fn default() -> Self {
        Self::new()
    }
}
impl ColorPage {
    #[must_use]
    pub fn new() -> Self {
        let obj: Self =
            Object::new(&[]).expect("Cannot create color page");
        obj
    }
}
