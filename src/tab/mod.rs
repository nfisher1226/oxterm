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
    pub fn label(&self) -> TabLabel {
        self.imp().label.clone()
    }

    pub fn terms(&self) -> RefCell<HashMap<String, Terminal>> {
        self.imp().terms.clone()
    }

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
}
