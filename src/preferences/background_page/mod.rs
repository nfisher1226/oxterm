mod imp;
mod color_page;
mod image_page;

use gtk::{
    glib::{self, GString, clone, Object},
    prelude::*,
    subclass::prelude::*,
};

pub use {color_page::ColorPage, image_page::ImagePage};

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
        obj.imp().background_type.connect_changed(clone!(@weak obj as bp => move |bt| {
            let imp = bp.imp();
            match bt.active_id().unwrap_or(GString::from("")).as_str() {
                "solid_color" => imp.stack.set_visible_child(&imp.color_page),
                "image" => imp.stack.set_visible_child(&imp.image_page),
                "gradient" => imp.stack.set_visible_child(&imp.gradient_editor),
                _ => {},
            }
        }));
        obj
    }
}
