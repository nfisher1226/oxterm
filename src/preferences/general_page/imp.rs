use gtk::{
    glib::{self, clone, subclass::InitializingObject},
    prelude::*,
    subclass::prelude::*,
    CompositeTemplate,
};

#[derive(CompositeTemplate, Default)]
#[template(file = "general_page.ui")]
pub struct GeneralPage {
    #[template_child]
    pub initial_title: TemplateChild<gtk::Entry>,
    #[template_child]
    pub dynamic_title: TemplateChild<gtk::ComboBoxText>,
    #[template_child]
    pub tab_position: TemplateChild<gtk::ComboBoxText>,
    #[template_child]
    pub wide_handles: TemplateChild<gtk::Switch>,
    #[template_child]
    pub custom_command_checkbutton: TemplateChild<gtk::CheckButton>,
    #[template_child]
    pub custom_command: TemplateChild<gtk::Entry>,
}

#[glib::object_subclass]
impl ObjectSubclass for GeneralPage {
    const NAME: &'static str = "GeneralPage";
    type Type = super::GeneralPage;
    type ParentType = gtk::Grid;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for GeneralPage {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        self.dynamic_title.set_active_id(Some("after_title"));
        self.tab_position.set_active_id(Some("top"));
        self.custom_command_checkbutton.connect_toggled(
            clone!(@strong self.custom_command as cc => move |but| {
                cc.set_sensitive(but.is_active());
            }),
        );
    }
}

impl WidgetImpl for GeneralPage {}
impl GridImpl for GeneralPage {}
