use {
    crate::TabLabel,
    gtk::{glib, subclass::prelude::*},
    std::{cell::RefCell, collections::HashMap},
    vte::Terminal,
};

#[derive(Default)]
pub struct Tab {
    pub label: TabLabel,
    pub terms: RefCell<HashMap<String, Terminal>>,
    pub current_term: RefCell<Option<String>>,
}

#[glib::object_subclass]
impl ObjectSubclass for Tab {
    const NAME: &'static str = "Tab";
    type Type = super::Tab;
    type ParentType = gtk::Box;
}

impl ObjectImpl for Tab {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
    }
}

impl WidgetImpl for Tab {}
impl BoxImpl for Tab {}
