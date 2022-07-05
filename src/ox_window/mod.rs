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
    #[must_use]
    pub fn new(app: &gtk::Application) -> Self {
        Object::new(&[("application", app)]).expect("Cannot create OxtermWindow")
    }

    #[must_use]
    pub fn notebook(&self) -> gtk::Notebook {
        self.imp().notebook.clone()
    }

    pub fn remove_tab(&self, tab: &Tab) {
        if let Some(num) = self.imp().notebook.page_num(tab) {
            self.imp().notebook.remove_page(Some(num));
        }
    }

    #[must_use]
    pub fn new_tab(&self) -> Tab {
        let tab = crate::Tab::new();
        self.imp().notebook.append_page(&tab, Some(&tab.label()));
        tab.label()
            .connect_close_clicked(clone!(@weak tab, @weak self as window => move |_| {
                window.remove_tab(&tab);
            }));
        if let Some(t) = tab.terms().borrow().values().next() {
            t.grab_focus();
        }
        tab.connect_close_tab(clone!(@weak self as window => move |tab| {
            window.remove_tab(tab);
        }));
        tab
    }

    fn current_page(&self) -> Option<u32> {
        self.imp().notebook.current_page()
    }

    #[must_use]
    pub fn current_tab(&self) -> Option<Tab> {
        self.imp()
            .notebook
            .nth_page(self.current_page())
            .map(|x| x.downcast::<Tab>().unwrap())
    }

    #[must_use]
    pub fn nth_tab(&self, num: u32) -> Option<Tab> {
        self.imp()
            .notebook
            .nth_page(Some(num))
            .map(|x| x.downcast::<Tab>().unwrap())
    }

    pub fn next_tab(&self) {
        if let Some(current) = self.imp().notebook.current_page() {
            let pages = self.imp().notebook.n_pages();
            if current == pages - 1 {
                self.imp().notebook.set_page(0);
            } else if let Ok(num) = i32::try_from(current + 1) {
                self.imp().notebook.set_page(num);
            }
        }
    }

    pub fn prev_tab(&self) {
        if let Some(current) = self.current_page() {
            let pages = self.imp().notebook.n_pages();
            if current == 0 {
                if let Ok(num) = i32::try_from(pages - 1) {
                    self.imp().notebook.set_page(num);
                }
            } else if let Ok(num) = i32::try_from(current - 1) {
                self.imp().notebook.set_page(num);
            }
        }
    }

    pub fn close_current_tab(&self) {
        self.imp().notebook.remove_page(self.current_page());
    }
}
