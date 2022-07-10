use gtk::{
    glib::{self, clone, GString, subclass::InitializingObject},
    prelude::*,
    subclass::prelude::*,
    CompositeTemplate,
};

#[derive(CompositeTemplate, Default)]
#[template(file = "image_page.ui")]
pub struct ImagePage {}

#[glib::object_subclass]
impl ObjectSubclass for ImagePage {
    const NAME: &'static str = "ImagePage";
    type Type = super::ImagePage;
    type ParentType = gtk::Grid;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for ImagePage {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
    }
}

impl WidgetImpl for ImagePage {}
impl GridImpl for ImagePage {}
