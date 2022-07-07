use gtk::{
    glib::{self, subclass::InitializingObject},
    prelude::*,
    subclass::prelude::*,
    CompositeTemplate,
};

#[derive(CompositeTemplate, Default)]
#[template(file = "preferences.ui")]
pub struct PreferencesDialog {
    #[template_child]
    pub stack: TemplateChild<gtk::Stack>,
    pub general_page: super::GeneralPage,
    pub text_page: super::TextPage,
    pub palette_page: super::PalettePage,
    pub background_page: super::BackgroundPage,
}

#[glib::object_subclass]
impl ObjectSubclass for PreferencesDialog {
    const NAME: &'static str = "PreferencesDialog";
    type Type = super::PreferencesDialog;
    type ParentType = gtk::Dialog;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for PreferencesDialog {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
    }
}

impl WidgetImpl for PreferencesDialog {}
impl DialogImpl for PreferencesDialog {}
impl WindowImpl for PreferencesDialog {}
