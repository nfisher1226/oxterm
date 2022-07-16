use std::fmt::format;

use crate::config::DynamicTitleStyle;

mod imp;

use {
    crate::{Tab, CONFIG},
    gtk::{
        gio,
        glib::{self, clone, Object},
        prelude::*,
        subclass::prelude::*,
    },
    std::path::PathBuf,
    vte::TerminalExt,
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
        let obj: Self = Object::new(&[("application", app)])
            .expect("Cannot create OxtermWindow");
        obj
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
    
    pub fn set_oxwindow_title(&self) {
        if let Some(term) = self.current_tab().map(|t| t.current_term()).flatten() {
            if let Some(path) = term.current_directory_uri().map(|d| PathBuf::from(d.as_str())) {
                if let Ok(cfg) = CONFIG.try_lock() {
                    self.set_title(Some(&match cfg.general.title_style {
                        DynamicTitleStyle::AfterTitle => format!(
                            "{} - {} ~ {}",
                            &cfg.general.initial_title,
                            env!("CARGO_PKG_VERSION"),
                            path.display(),
                        ),
                        DynamicTitleStyle::BeforeTitle => format!(
                            "{} ~ {} - {}",
                            path.display(),
                            &cfg.general.initial_title,
                            env!("CARGO_PKG_VERSION"),
                        ),
                        DynamicTitleStyle::ReplacesTitle => format!(
                            "{}",
                            path.display(),
                        ),
                        DynamicTitleStyle::NotDisplayed => format!(
                            "{} - {}",
                            &cfg.general.initial_title,
                            env!("CARGO_PKG_VERSION"),
                        )
                    }));
                }
            }
        }
    }

    pub fn apply_config(&self) {
        self.set_oxwindow_title();
        if let Ok(cfg) = CONFIG.try_lock() {
            self.imp()
                .notebook
                .set_tab_pos(cfg.general.tab_position.into());
        }
        for obj in self.imp().notebook.pages().into_iter() {
            if let Ok(page) = obj.downcast::<gtk::NotebookPage>() {
                if let Ok(tab) = page.child().downcast::<Tab>() {
                    tab.apply_config();
                }
            }
        }
    }
}
