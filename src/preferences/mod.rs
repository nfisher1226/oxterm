mod gradient_editor;
mod imp;

use {
    crate::OxWindow,
    gtk::{
        glib::{self, clone, Object},
        prelude::*,
        subclass::prelude::*,
    },
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
        Object::new(&[("use-header-bar", &1)]).expect("Cannot create preferences dialog")
    }
}
