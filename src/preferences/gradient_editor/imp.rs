use {
    super::stop_editor::StopEditor,
    gtk::{
        glib::{self, subclass::InitializingObject},
        prelude::*,
        subclass::prelude::*,
        CompositeTemplate,
    },
    std::{cell::RefCell, collections::HashMap},
};

#[derive(CompositeTemplate, Default)]
#[template(file = "gradient_editor.ui")]
pub struct GradientEditor {
    #[template_child]
    pub gradient_kind: TemplateChild<gtk::ComboBoxText>,
    #[template_child]
    pub position_type_stack: TemplateChild<gtk::Stack>,
    #[template_child]
    pub direction_type: TemplateChild<gtk::ComboBoxText>,
    #[template_child]
    pub direction_stack: TemplateChild<gtk::Stack>,
    #[template_child]
    pub degrees: TemplateChild<gtk::SpinButton>,
    #[template_child]
    pub vertical_position: TemplateChild<gtk::ComboBoxText>,
    #[template_child]
    pub horizontal_position: TemplateChild<gtk::ComboBoxText>,
    #[template_child]
    pub num_stops: TemplateChild<gtk::SpinButton>,
    #[template_child]
    pub stop_selector: TemplateChild<gtk::ComboBoxText>,
    #[template_child]
    pub stops_stack: TemplateChild<gtk::Stack>,
    pub stops: RefCell<HashMap<String, StopEditor>>,
}

#[glib::object_subclass]
impl ObjectSubclass for GradientEditor {
    const NAME: &'static str = "GradientEditor";
    type Type = super::GradientEditor;
    type ParentType = gtk::Grid;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for GradientEditor {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        self.gradient_kind.set_active_id(Some("linear"));
        self.direction_type.set_active_id(Some("angle"));
        self.vertical_position.set_active_id(Some("top"));
        self.horizontal_position.set_active_id(Some("left"));
    }
}

impl WidgetImpl for GradientEditor {}
impl GridImpl for GradientEditor {}
