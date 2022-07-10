use {
    crate::preferences::GradientEditor,
    gtk::{
        glib::{self, clone, GString, subclass::InitializingObject},
        prelude::*,
        subclass::prelude::*,
        CompositeTemplate,
    },
    super::{ColorPage, ImagePage},
};

#[derive(CompositeTemplate, Default)]
#[template(file = "background_page.ui")]
pub struct BackgroundPage {
    #[template_child]
    pub background_type: TemplateChild<gtk::ComboBoxText>,
    #[template_child]
    pub transparency: TemplateChild<gtk::Scale>,
    #[template_child]
    pub stack: TemplateChild<gtk::Stack>,
    pub color_page: ColorPage,
    pub image_page: ImagePage,
    pub gradient_editor: GradientEditor,
}

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
        self.stack.add_child(&self.color_page);
        self.stack.add_child(&self.image_page);
        self.stack.add_child(&self.gradient_editor);
        self.background_type.set_active_id(Some("solid_color"));
    }
}

impl WidgetImpl for BackgroundPage {}
impl GridImpl for BackgroundPage {}
