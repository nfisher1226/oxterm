mod imp;

use {
    gtk::{
        glib::{self, Object},
        prelude::*,
        subclass::prelude::*,
    },
    gtk4_file_chooser_button::FileChooserButton,
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
        let obj: Self = Object::new(&[]).expect("Cannot create background page");
        obj
    }

    pub fn image_file(&self) -> FileChooserButton {
        self.imp().image_file.clone()
    }
}
