use {
    crate::Tab,
    gtk::{
        glib::{self, subclass::InitializingObject},
        prelude::*,
        subclass::prelude::*,
        CompositeTemplate,
    },
    std::{cell::RefCell, collections::HashMap},
};

#[derive(CompositeTemplate, Default)]
#[template(file = "ox_window.ui")]
pub struct OxWindow {
    #[template_child]
    pub notebook: TemplateChild<gtk::Notebook>,
    #[template_child]
    pub menu_button: TemplateChild<gtk::MenuButton>,
    pub tabs: RefCell<HashMap<String, Tab>>,
}

#[glib::object_subclass]
impl ObjectSubclass for OxWindow {
    const NAME: &'static str = "OxWindow";
    type Type = super::OxWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for OxWindow {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
    }
}

impl ApplicationWindowImpl for OxWindow {}
impl WindowImpl for OxWindow {}
impl WidgetImpl for OxWindow {}
impl BoxImpl for OxWindow {}
