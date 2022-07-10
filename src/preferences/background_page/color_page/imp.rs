use gtk::{
    glib::{self, clone, GString, subclass::InitializingObject},
    prelude::*,
    subclass::prelude::*,
    CompositeTemplate,
};

#[derive(CompositeTemplate, Default)]
#[template(file = "color_page.ui")]
pub struct ColorPage {
    #[template_child]
    pub color_type: TemplateChild<gtk::ComboBoxText>,
    #[template_child]
    pub color_button: TemplateChild<gtk::ColorButton>,
}

#[glib::object_subclass]
impl ObjectSubclass for ColorPage {
    const NAME: &'static str = "ColorPage";
    type Type = super::ColorPage;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for ColorPage {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        self.color_type.set_active_id(Some("black"));
        self.color_type.connect_changed(clone!(@strong self.color_button as but => move |ct| {
            match ct.active_id().unwrap_or(GString::from("")).as_str() {
                "custom" => but.set_sensitive(true),
                _ => but.set_sensitive(false),
            }
        }));
    }
}

impl WidgetImpl for ColorPage {}
impl BoxImpl for ColorPage {}
