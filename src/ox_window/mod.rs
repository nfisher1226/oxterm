use crate::config::Background;

mod imp;

use {
    crate::{
        config::{AsCss, DynamicTitleStyle},
        Tab, CONFIG,
    },
    gtk::{
        gdk::Display,
        gio,
        glib::{self, clone, Object},
        prelude::*,
        subclass::prelude::*,
        CssProvider, StyleContext,
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
        let obj: Self = Object::new(&[("application", app)]).expect("Cannot create OxtermWindow");
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

    /// Gets the current `Tab` widget
    /// # Panics
    /// Because we will never have a notebook page which is not a `Tab`, it is
    /// safe to call unwrap here and it will never panic
    #[must_use]
    pub fn current_tab(&self) -> Option<Tab> {
        self.imp()
            .notebook
            .nth_page(self.current_page())
            .map(|x| x.downcast::<Tab>().unwrap())
    }

    /// Gets the tab at index `num`
    /// # Panics
    /// Because we will never have a notebook page which is not a `Tab`, it is
    /// safe to call unwrap here and it will never panic
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
        if let Some(term) = self.current_tab().and_then(|t| t.current_term()) {
            if let Some(path) = term
                .current_directory_uri()
                .map(|d| PathBuf::from(d.as_str()))
            {
                let path = path.to_string_lossy();
                let path = path.strip_prefix("file://").unwrap_or(&path);
                if let Ok(cfg) = CONFIG.try_lock() {
                    let gen = cfg.general.clone();
                    self.set_title(Some(&match gen.title_style {
                        DynamicTitleStyle::AfterTitle => format!(
                            "{}-{} ~ {}",
                            &gen.initial_title,
                            env!("CARGO_PKG_VERSION"),
                            path,
                        ),
                        DynamicTitleStyle::BeforeTitle => format!(
                            "{} ~ {}-{}",
                            path,
                            &gen.initial_title,
                            env!("CARGO_PKG_VERSION"),
                        ),
                        DynamicTitleStyle::ReplacesTitle => path.to_string(),
                        DynamicTitleStyle::NotDisplayed => {
                            format!("{}-{}", &gen.initial_title, env!("CARGO_PKG_VERSION"),)
                        }
                    }));
                }
            }
        }
    }

    pub fn set_css(&self) {
        let bg = {
            let cfg = CONFIG.try_lock();
            if let Ok(cfg) = cfg {
                cfg.background.clone()
            } else {
                Background::default()
            }
        };
        let css = bg.as_css();
        let provider = CssProvider::new();
        provider.load_from_data(css.as_bytes());
        StyleContext::add_provider_for_display(
            &Display::default().expect("Cannot get display"),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    pub fn apply_config(&self) {
        self.set_oxwindow_title();
        self.set_css();
        if let Ok(cfg) = CONFIG.try_lock() {
            self.imp()
                .notebook
                .set_tab_pos(cfg.general.tab_position.into());
        }
        for obj in self.imp().notebook.pages() {
            if let Ok(page) = obj.downcast::<gtk::NotebookPage>() {
                if let Ok(tab) = page.child().downcast::<Tab>() {
                    tab.apply_config();
                }
            }
        }
    }

    pub fn all_windows(&self) -> Option<Vec<Self>> {
        if let Some(app) = self.application() {
            let mut windows: Vec<OxWindow> = vec![];
            for window in app.windows() {
                if let Ok(w) = window.downcast::<OxWindow>() {
                    windows.push(w);
                }
            }
            Some(windows)
        } else {
            None
        }
    }
}
