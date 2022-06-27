mod imp;

use {
    crate::Tab,
    gtk::{
        gio,
        glib::{self, clone, Object},
        prelude::*,
        subclass::prelude::*,
    },
};

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

    pub fn remove_tab(&self, tab: &Tab) {
        let _val = self
            .imp()
            .tabs
            .borrow_mut()
            .remove(tab.widget_name().as_str());
        if let Some(num) = self.imp().notebook.page_num(tab) {
            self.imp().notebook.remove_page(Some(num));
        }
    }

    pub fn new_tab(&self) -> Tab {
        let tab = crate::Tab::new();
        self.imp().notebook.append_page(&tab, Some(&tab.label()));
        self.imp()
            .tabs
            .borrow_mut()
            .insert(tab.widget_name().to_string(), tab.clone());
        tab.label()
            .connect_close_clicked(clone!(@weak tab, @weak self as window => move |_| {
                window.remove_tab(&tab);
            }));
        tab.terms().borrow().values().next().unwrap().grab_focus();
        tab
    }

    fn current_page(&self) -> Option<u32> {
        self.imp().notebook.current_page()
    }

    pub fn current_tab(&self) -> Option<Tab> {
        if let Some(t) = self.imp().notebook.nth_page(self.current_page()) {
            self.imp()
                .tabs
                .borrow()
                .get(&t.widget_name().to_string())
                .cloned()
        } else {
            None
        }
    }

    pub fn nth_tab(&self, num: u32) -> Option<Tab> {
        if let Some(t) = self.imp().notebook.nth_page(Some(num)) {
            self.imp()
                .tabs
                .borrow()
                .get(&t.widget_name().to_string())
                .cloned()
        } else {
            None
        }
    }

    pub fn next_tab(&self) {
        if let Some(current) = self.imp().notebook.current_page() {
            let pages = self.imp().notebook.n_pages();
            if current == pages - 1 {
                self.imp().notebook.set_page(0);
            } else {
                self.imp()
                    .notebook
                    .set_page((current + 1).try_into().unwrap());
            }
        }
    }

    pub fn prev_tab(&self) {
        if let Some(current) = self.current_page() {
            let pages = self.imp().notebook.n_pages();
            if current == 0 {
                self.imp()
                    .notebook
                    .set_page((pages - 1).try_into().unwrap());
            } else {
                self.imp()
                    .notebook
                    .set_page((current - 1).try_into().unwrap());
            }
        }
    }

    pub fn close_current_tab(&self) {
        if let Some(page) = self.current_page() {
            if let Some(tab) = self.current_tab() {
                let name = tab.widget_name().to_string();
                self.imp().tabs.borrow_mut().remove(&name);
            }
            self.imp().notebook.remove_page(Some(page));
        }
    }
}
