use gtk::{
    glib::{self, clone, subclass::InitializingObject, GString},
    prelude::*,
    subclass::prelude::*,
    CompositeTemplate,
};

#[derive(CompositeTemplate, Default)]
#[template(file = "text_page.ui")]
pub struct TextPage {
    #[template_child]
    pub cursor_style: TemplateChild<gtk::ComboBoxText>,
    #[template_child]
    pub cursor_blinks: TemplateChild<gtk::CheckButton>,
    #[template_child]
    pub scrollback_lines: TemplateChild<gtk::SpinButton>,
    #[template_child]
    pub infinite_scrollback: TemplateChild<gtk::CheckButton>,
    #[template_child]
    pub system_font: TemplateChild<gtk::CheckButton>,
    #[template_child]
    pub font_chooser_button: TemplateChild<gtk::FontButton>,
    #[template_child]
    pub color_type: TemplateChild<gtk::ComboBoxText>,
    #[template_child]
    pub text_color: TemplateChild<gtk::ColorButton>,
}

#[glib::object_subclass]
impl ObjectSubclass for TextPage {
    const NAME: &'static str = "TextPage";
    type Type = super::TextPage;
    type ParentType = gtk::Grid;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for TextPage {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        self.cursor_style.set_active_id(Some("block"));
        self.infinite_scrollback.connect_toggled(
            clone!(@strong self.scrollback_lines as sl => move |but| {
                sl.set_sensitive(!but.is_active());
            }),
        );
        self.system_font.connect_toggled(
            clone!(@strong self.font_chooser_button as fc => move |but| {
                fc.set_sensitive(!but.is_active());
            }),
        );
        self.text_color.set_sensitive(false);
        self.color_type.set_active_id(Some("white"));
        self.color_type
            .connect_changed(clone!(@strong self.text_color as tc => move |bx| {
                match bx.active_id().unwrap_or(GString::from("white")).as_str() {
                    "custom" => tc.set_sensitive(true),
                    _ => tc.set_sensitive(false),
                }
            }));
    }
}

impl WidgetImpl for TextPage {}
impl GridImpl for TextPage {}
