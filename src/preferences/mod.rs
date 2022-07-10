mod background_page;
mod general_page;
mod gradient_editor;
mod imp;
mod palette_page;
mod text_page;

use {
    crate::{config::Config, OxWindow, Values},
    gtk::{
        glib::{self, clone, Object},
        prelude::*,
        subclass::prelude::*,
    },
};
pub use {
    background_page::BackgroundPage, general_page::GeneralPage,
    gradient_editor::GradientEditor, palette_page::PalettePage,
    text_page::TextPage,
};

glib::wrapper! {
    pub struct PreferencesDialog(ObjectSubclass<imp::PreferencesDialog>)
        @extends gtk::Dialog, gtk::Widget, gtk::Window,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
            gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Default for PreferencesDialog {
    fn default() -> Self {
        Self::new()
    }
}

pub fn run(window: &OxWindow) {
    let dlg = PreferencesDialog::new();
    dlg.set_transient_for(Some(window));
    dlg.set_modal(true);
    dlg.connect_response(clone!(@weak window => move |dlg, res| {
        dlg.close();
    }));
    dlg.show();
}

impl PreferencesDialog {
    #[must_use]
    pub fn new() -> Self {
        let obj: Self =
            Object::new(&[("use-header-bar", &1)]).expect("Cannot create preferences dialog");
        obj.imp()
            .stack
            .add_titled(&obj.imp().general_page, Some("general"), "General");
        obj.imp()
            .stack
            .add_titled(&obj.imp().text_page, Some("text"), "Text");
        obj.imp()
            .stack
            .add_titled(&obj.imp().palette_page, Some("palette"), "Palette");
        obj.imp()
            .stack
            .add_titled(&obj.imp().background_page, Some("background"), "Background");
        obj
    }
}

impl Values<Config> for PreferencesDialog {
    fn values(&self) -> Config {
        let imp = self.imp();
        Config {
            general: imp.general_page.values(),
            text: imp.text_page.values(),
        }
    }

    fn set_values(&self, cfg: &Config) {
        let imp = self.imp();
        imp.general_page.set_values(&cfg.general);
        imp.text_page.set_values(&cfg.text);
    }
}
