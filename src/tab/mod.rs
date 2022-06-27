mod imp;

use {
    crate::{TabLabel, SHELL},
    gtk::{
        gio::Cancellable,
        glib::{self, Object, SpawnFlags},
        prelude::*,
        subclass::prelude::*,
    },
    std::{cell::RefCell, collections::HashMap, path::Path},
    vte::{PtyFlags, Terminal, TerminalExt},
};

glib::wrapper! {
    pub struct Tab(ObjectSubclass<imp::Tab>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for Tab {
    fn default() -> Self {
        Self::new()
    }
}

impl Tab {
    pub fn new() -> Self {
        let name: String = std::iter::repeat_with(fastrand::alphanumeric)
            .take(10)
            .collect();
        let tab: Self = Object::new(&[
            ("orientation", &gtk::Orientation::Horizontal),
            ("name", &name),
            ("hexpand", &true),
            ("halign", &gtk::Align::Fill),
        ])
        .expect("Cannot create tab");
        let term = Self::new_term();
        tab.append(&term);
        term.show();
        tab.imp()
            .terms
            .borrow_mut()
            .insert(term.widget_name().to_string(), term.clone());
        tab
    }

    pub fn label(&self) -> TabLabel {
        self.imp().label.clone()
    }

    pub fn terms(&self) -> RefCell<HashMap<String, Terminal>> {
        self.imp().terms.clone()
    }

    pub fn new_term() -> Terminal {
        let term = Terminal::new();
        let name: String = std::iter::repeat_with(fastrand::alphanumeric)
            .take(10)
            .collect();
        term.set_widget_name(&name);
        term.set_hexpand(true);
        term.set_halign(gtk::Align::Fill);
        term.spawn_async(
            PtyFlags::DEFAULT,
            None,
            &[Path::new(*SHELL)],
            &[Path::new("TERM=xterm")],
            SpawnFlags::DEFAULT,
            Some(Box::new(|| {})),
            10,
            Some(&Cancellable::new()),
            None,
        );
        term
    }

    pub fn split(&self, orientation: Option<gtk::Orientation>) {
        let len = { self.imp().terms.borrow().iter().len() };
        match len {
            1 => self.first_split(orientation),
            _ => {}
        }
    }

    fn first_split(&self, orientation: Option<gtk::Orientation>) {
        let mut terms = self.imp().terms.borrow_mut();
        let term0 = terms.values().next().unwrap().clone();
        self.remove(&term0);
        let term1 = Self::new_term();
        terms.insert(term1.widget_name().to_string(), term1.clone());
        let orientation = match orientation {
            Some(o) => o,
            None => gtk::Orientation::Horizontal,
        };
        let paned = gtk::Paned::builder()
            .orientation(orientation)
            .start_child(&term0)
            .end_child(&term1)
            .build();
        self.append(&paned);
        paned.show();
    }
}
