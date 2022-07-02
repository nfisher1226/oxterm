use gtk::{
    glib::{
        self,
        subclass::InitializingObject,
    },
    prelude::*,
    subclass::prelude::*,
    CompositeTemplate,
};

#[derive(CompositeTemplate, Default)]
#[template(file = "stop_editor.ui")]
pub struct StopEditor {
    #[template_child]
    pub button: TemplateChild<gtk::ColorButton>,
    #[template_child]
    pub scale: TemplateChild<gtk::SpinButton>,
}

#[glib::object_subclass]
impl ObjectSubclass for StopEditor {
    const NAME: &'static str = "StopEditor";
    type Type = super::StopEditor;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for StopEditor {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
    }
}

impl WidgetImpl for StopEditor {}
impl BoxImpl for StopEditor {}
