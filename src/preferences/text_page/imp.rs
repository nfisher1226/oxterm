use gtk::{
    glib::{self, subclass::InitializingObject},
    prelude::*,
    subclass::prelude::*,
    CompositeTemplate,
};

#[derive(CompositeTemplate, Default)]
#[template(file = "text_page.ui")]
pub struct TextPage {}

#[glib::object_subclass]
impl ObjectSubclass for TextPage {
    const NAME: &'static str = "TextPage";
    type Type = super::TextPage;
    type ParentType = gtk::Grid;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for TextPage {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
    }
}

impl WidgetImpl for TextPage {}
impl GridImpl for TextPage {}
