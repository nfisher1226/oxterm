mod imp;

use gtk::{glib::{self, Object}, gio, subclass::prelude::*};

glib::wrapper! {
    pub struct OxWindow(ObjectSubclass<imp::OxWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible,
            gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root,
            gtk::ShortcutManager;
}

impl OxWindow {
    pub fn new(app: &gtk::Application) -> Self {
        Object::new(&[("application", app)]).expect("Cannot create OxtermWindow")
    }

    pub fn notebook(&self) -> gtk::Notebook {
        self.imp().notebook.clone()
    }
}
