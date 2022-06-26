use {
    gtk::{
        glib::{self, subclass::{InitializingObject, Signal}},
        prelude::*,
        subclass::prelude::*,
        CompositeTemplate,
    },
    once_cell::sync::Lazy,
};

#[derive(CompositeTemplate, Default)]
#[template(file = "tab_label.ui")]
pub struct TabLabel {
    #[template_child]
    pub label: TemplateChild<gtk::Label>,
    #[template_child]
    pub button: TemplateChild<gtk::Button>,
}

#[glib::object_subclass]
impl ObjectSubclass for TabLabel {
    const NAME: &'static str = "TabLabel";
    type Type = super::TabLabel;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for TabLabel {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        let label = obj.clone();
        self.button.connect_clicked(move |_| {
            label.emit_by_name::<()>("close-clicked", &[]);
        });
    }

    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![
                Signal::builder(
                    "close-clicked",
                    &[],
                    <()>::static_type().into(),
                )
                .build()
            ]
        });
        SIGNALS.as_ref()
    }
}

impl WidgetImpl for TabLabel {}
impl BoxImpl for TabLabel {}
