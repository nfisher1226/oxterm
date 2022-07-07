use gtk::{
    glib::{self, subclass::InitializingObject},
    prelude::*,
    subclass::prelude::*,
    CompositeTemplate,
};

#[derive(CompositeTemplate, Default)]
#[template(file = "palette_page.ui")]
pub struct PalettePage {
    #[template_child]
    pub palette_selector: TemplateChild<gtk::ComboBoxText>,
    #[template_child]
    pub add_palette: TemplateChild<gtk::Button>,
    #[template_child]
    pub remove_palette: TemplateChild<gtk::Button>,
    #[template_child]
    pub black_color: TemplateChild<gtk::ColorButton>,
    #[template_child]
    pub red_color: TemplateChild<gtk::ColorButton>,
    #[template_child]
    pub green_color: TemplateChild<gtk::ColorButton>,
    #[template_child]
    pub brown_color: TemplateChild<gtk::ColorButton>,
    #[template_child]
    pub blue_color: TemplateChild<gtk::ColorButton>,
    #[template_child]
    pub magenta_color: TemplateChild<gtk::ColorButton>,
    #[template_child]
    pub cyan_color: TemplateChild<gtk::ColorButton>,
    #[template_child]
    pub light_grey_color: TemplateChild<gtk::ColorButton>,
    #[template_child]
    pub dark_grey_color: TemplateChild<gtk::ColorButton>,
    #[template_child]
    pub light_red_color: TemplateChild<gtk::ColorButton>,
    #[template_child]
    pub light_green_color: TemplateChild<gtk::ColorButton>,
    #[template_child]
    pub yellow_color: TemplateChild<gtk::ColorButton>,
    #[template_child]
    pub light_magenta_color: TemplateChild<gtk::ColorButton>,
    #[template_child]
    pub light_cyan_color: TemplateChild<gtk::ColorButton>,
    #[template_child]
    pub white_color: TemplateChild<gtk::ColorButton>,
}

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
        self.palette_selector.set_active_id(Some("default"));
    }
}

impl WidgetImpl for PalettePage {}
impl GridImpl for PalettePage {}
