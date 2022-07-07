use gtk::{
    glib::{self, subclass::InitializingObject},
    prelude::*,
    subclass::prelude::*,
    CompositeTemplate,
};

#[derive(CompositeTemplate, Default)]
#[template(file = "palette_page.ui")]
pub struct PalettePage {}

#[glib::object_subclass]
impl ObjectSubclass for PalettePage {
    const NAME: &'static str = "PalettePage";
    type Type = super::PalettePage;
    type ParentType = gtk::Grid;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for PalettePage {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
    }
}

impl WidgetImpl for PalettePage {}
impl GridImpl for PalettePage {}
