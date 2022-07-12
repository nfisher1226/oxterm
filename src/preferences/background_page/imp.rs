use {
    crate::preferences::GradientEditor,
    gtk::{
        glib::{self, clone, subclass::InitializingObject, GString},
        prelude::*,
        subclass::prelude::*,
        CompositeTemplate,
    },
    gtk4_file_chooser_button::FileChooserButton,
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
    #[template_child]
    pub color_type: TemplateChild<gtk::ComboBoxText>,
    #[template_child]
    pub color_button: TemplateChild<gtk::ColorButton>,
    #[template_child]
    pub image_grid: TemplateChild<gtk::Grid>,
    #[template_child]
    pub image_style: TemplateChild<gtk::ComboBoxText>,
    pub image_file: FileChooserButton,
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
        self.color_type.set_active_id(Some("black"));
        self.color_type
            .connect_changed(clone!(@strong self.color_button as but => move |ct| {
                match ct.active_id().unwrap_or(GString::from("")).as_str() {
                    "custom" => but.set_sensitive(true),
                    _ => but.set_sensitive(false),
                }
            }));
        self.stack
            .add_named(&self.gradient_editor, Some("gradient"));
        self.background_type.set_active_id(Some("solid_color"));
        self.background_type
            .connect_changed(clone!(@strong self.stack as stack => move |btype| {
                if let Some(name) = btype.active_id() {
                    stack.set_visible_child_name(name.as_str());
                }
            }));
        self.image_style.set_active_id(Some("tiled"));
        self.image_grid.attach(&self.image_file, 1, 0, 1, 1);
        self.image_file
            .set_tooltip_text(Some("Choose and image file to use as a background"));
    }
}

impl WidgetImpl for BackgroundPage {}
impl GridImpl for BackgroundPage {}
