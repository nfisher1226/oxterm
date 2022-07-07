use gtk::{
    glib::{self, subclass::InitializingObject},
    prelude::*,
    subclass::prelude::*,
    CompositeTemplate,
};

#[derive(CompositeTemplate, Default)]
#[template(file = "background_page.ui")]
pub struct BackgroundPage {}

#[glib::object_subclass]
impl ObjectSubclass for BackgroundPage {
    const NAME: &'static str = "BackgroundPage";
    type Type = super::BackgroundPage;
    type ParentType = gtk::Grid;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for BackgroundPage {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
    }
}

impl WidgetImpl for BackgroundPage {}
impl GridImpl for BackgroundPage {}
