mod imp;

use gtk::{
    glib::{self, clone, Object},
    prelude::*,
    subclass::prelude::*,
};

glib::wrapper! {
    pub struct ImagePage(ObjectSubclass<imp::ImagePage>)
        @extends gtk::Dialog, gtk::Widget, gtk::Window,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
            gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Default for ImagePage {
    fn default() -> Self {
        Self::new()
    }
}

impl ImagePage {
    #[must_use]
    pub fn new() -> Self {
        let obj: Self =
            Object::new(&[]).expect("Cannot create image page");
        obj
    }
}
