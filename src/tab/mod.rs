use crate::OxWindow;

mod imp;

use {
    crate::{
        config::{Background, BackgroundColor, Scrollback, TextColor},
        TabLabel, CONFIG, SHELL,
    },
    gtk::{
        gio::Cancellable,
        glib::{self, clone, Object, SpawnFlags},
        prelude::*,
        subclass::prelude::*,
        traits::WidgetExt,
    },
    std::{cell::RefCell, collections::HashMap, path::Path},
    vte::{PtyFlags, Terminal, TerminalExt, TerminalExtManual},
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
    #[must_use]
    pub fn new() -> Self {
        let tab: Self = Object::new(&[
            ("orientation", &gtk::Orientation::Horizontal),
            ("hexpand", &true),
            ("halign", &gtk::Align::Fill),
        ])
        .expect("Cannot create tab");
        let term = tab.new_term();
        tab.append(&term);
        term.show();
        tab.imp()
            .terms
            .borrow_mut()
            .insert(term.widget_name().to_string(), term);
        tab.apply_config();
        tab
    }

    #[must_use]
    pub fn label(&self) -> TabLabel {
        self.imp().label.clone()
    }

    #[must_use]
    pub fn terms(&self) -> RefCell<HashMap<String, Terminal>> {
        self.imp().terms.clone()
    }

    #[must_use]
    pub fn new_term(&self) -> Terminal {
        let term = Terminal::new();
        let name: String = std::iter::repeat_with(fastrand::alphanumeric)
            .take(10)
            .collect();
        term.set_widget_name(&name);
        term.set_hexpand(true);
        term.set_halign(gtk::Align::Fill);
        let cn: Option<&Cancellable> = None;
        term.spawn_async(
            PtyFlags::DEFAULT,
            None,
            &[Path::new(*SHELL)],
            &[Path::new("TERM=xterm")],
            SpawnFlags::DEFAULT,
            Some(Box::new(|| {})),
            10,
            cn,
            None,
        );
        term.connect_has_focus_notify(clone!(@weak self as tab => move |term| {
            *tab.imp().current_term.borrow_mut() = Some(term.widget_name().to_string());
        }));
        term.connect_current_directory_uri_changed(clone!(@weak self as tab => move |term| {
            if let Some(window) = tab.root() {
                if let Ok(oxwindow) = window.downcast::<OxWindow>() {
                    oxwindow.set_oxwindow_title();
                }
            }
        }));
        term.connect_child_exited(clone!(@weak self as tab => move |term,_| {
            tab.close_term(term);
        }));
        term
    }

    #[must_use]
    pub fn current_term(&self) -> Option<Terminal> {
        if let Some(name) = &*self.imp().current_term.borrow() {
            return self.imp().terms.borrow().get(name).cloned();
        }
        None
    }

    fn current_term_parent(&self) -> Option<(Terminal, gtk::Paned)> {
        if let Some(term) = self.current_term() {
            if let Some(paned) = term.parent() {
                if let Ok(paned) = paned.downcast::<gtk::Paned>() {
                    return Some((term, paned));
                }
            }
        }
        None
    }

    pub fn split(&self, orientation: Option<gtk::Orientation>) {
        let len = { self.imp().terms.borrow().iter().len() };
        match len {
            1 => self.first_split(orientation),
            _ => self.split_again(orientation),
        }
        self.apply_config();
    }

    fn first_split(&self, orientation: Option<gtk::Orientation>) {
        let mut terms = self.imp().terms.borrow_mut();
        let old_term = terms.values().next().unwrap().clone();
        self.remove(&old_term);
        let new_term = self.new_term();
        terms.insert(new_term.widget_name().to_string(), new_term.clone());
        let orientation = match orientation {
            Some(o) => o,
            None => gtk::Orientation::Horizontal,
        };
        let paned = gtk::Paned::builder()
            .orientation(orientation)
            .halign(gtk::Align::Fill)
            .hexpand(true)
            .start_child(&old_term)
            .end_child(&new_term)
            .wide_handle(CONFIG.try_lock().unwrap().general.wide_handles)
            .build();
        self.append(&paned);
        paned.show();
    }

    fn split_again(&self, orientation: Option<gtk::Orientation>) {
        if let Some((term0, paned0)) = self.current_term_parent() {
            let child1 = paned0.start_child();
            let child2 = paned0.end_child();
            let term1 = self.new_term();
            match (child1, child2) {
                (Some(_), None) => {
                    paned0.set_end_child(Some(&term1));
                    if let Some(o) = orientation {
                        paned0.set_orientation(o);
                    }
                }
                (None, Some(_)) => {
                    paned0.set_start_child(Some(&term1));
                    if let Some(o) = orientation {
                        paned0.set_orientation(o);
                    }
                }
                (Some(t0), Some(t1)) => match term0.widget_name().as_str() {
                    s if s == t0.widget_name().as_str() => {
                        let pos = paned0.position();
                        let ch: Option<&gtk::Widget> = None;
                        paned0.set_start_child(ch);
                        let paned1 = gtk::Paned::builder()
                            .orientation(orientation.unwrap_or(gtk::Orientation::Horizontal))
                            .halign(gtk::Align::Fill)
                            .hexpand(true)
                            .start_child(&t0)
                            .end_child(&term1)
                            .wide_handle(CONFIG.try_lock().unwrap().general.wide_handles)
                            .build();
                        paned0.set_start_child(Some(&paned1));
                        paned1.show();
                        paned0.set_position(pos);
                        t0.grab_focus();
                    }
                    s if s == t1.widget_name().as_str() => {
                        let pos = paned0.position();
                        let ch: Option<&gtk::Widget> = None;
                        paned0.set_end_child(ch);
                        let paned1 = gtk::Paned::builder()
                            .orientation(orientation.unwrap_or(gtk::Orientation::Horizontal))
                            .halign(gtk::Align::Fill)
                            .hexpand(true)
                            .start_child(&t1)
                            .end_child(&term1)
                            .wide_handle(CONFIG.try_lock().unwrap().general.wide_handles)
                            .build();
                        paned0.set_end_child(Some(&paned1));
                        paned1.show();
                        paned0.set_position(pos);
                        t1.grab_focus();
                    }
                    _ => {}
                },
                (None, None) => {}
            }
        }
    }

    pub fn close_term(&self, term: &Terminal) {
        if let Some(parent) = term.parent() {
            if let Ok(paned) = parent.clone().downcast::<gtk::Paned>() {
                let name = term.widget_name();
                let ch: Option<&gtk::Widget> = None;
                if let Some(t) = paned.start_child() {
                    if t.widget_name() == name {
                        paned.set_start_child(ch);
                        self.imp()
                            .terms
                            .borrow_mut()
                            .remove(t.widget_name().as_str());
                        if let Some(t) = paned.end_child() {
                            if let Some(parent) = paned.parent() {
                                if let Ok(tab) = parent.clone().downcast::<Tab>() {
                                    paned.set_end_child(ch);
                                    tab.remove(&paned);
                                    tab.append(&t);
                                } else if let Ok(parent) = parent.downcast::<gtk::Paned>() {
                                    if let Some(w) = parent.start_child() {
                                        if paned.widget_name() == w.widget_name() {
                                            paned.set_end_child(ch);
                                            parent.set_start_child(Some(&t));
                                        }
                                    }
                                    if let Some(w) = parent.end_child() {
                                        if paned.widget_name() == w.widget_name() {
                                            paned.set_end_child(ch);
                                            parent.set_end_child(Some(&t));
                                        }
                                    }
                                }
                            }
                        }
                        return;
                    }
                }
                if let Some(t) = paned.end_child() {
                    if t.widget_name() == name {
                        paned.set_end_child(ch);
                        self.imp()
                            .terms
                            .borrow_mut()
                            .remove(t.widget_name().as_str());
                        if let Some(t) = paned.start_child() {
                            if let Some(parent) = paned.parent() {
                                if let Ok(tab) = parent.clone().downcast::<Tab>() {
                                    paned.set_start_child(ch);
                                    tab.remove(&paned);
                                    tab.append(&t);
                                } else if let Ok(parent) = parent.downcast::<gtk::Paned>() {
                                    if let Some(w) = parent.start_child() {
                                        if paned.widget_name() == w.widget_name() {
                                            paned.set_end_child(ch);
                                            parent.set_start_child(Some(&t));
                                        }
                                    }
                                    if let Some(w) = parent.end_child() {
                                        if paned.widget_name() == w.widget_name() {
                                            paned.set_end_child(ch);
                                            parent.set_end_child(Some(&t));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else if let Ok(tab) = parent.downcast::<Tab>() {
                tab.remove(term);
                tab.emit_by_name::<()>("close-tab", &[]);
            }
        }
    }

    /// Connect to the "close-tab" signal, emitted when the last terminal has closed
    /// # Panics
    /// Panics if unable to get the object from the emitted signal (impossible)
    pub fn connect_close_tab<F: Fn(&Self) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_local("close-tab", true, move |values| {
            let obj = values[0].get::<Self>().unwrap();
            f(&obj);
            None
        })
    }

    pub fn apply_config(&self) {
        if let Ok(cfg) = CONFIG.try_lock() {
            for term in self.imp().terms.borrow().values() {
                if let Some(parent) = term.parent() {
                    if let Ok(paned) = parent.downcast::<gtk::Paned>() {
                        paned.set_wide_handle(cfg.general.wide_handles);
                    }
                }
                term.set_cursor_shape(cfg.text.cursor.style.into());
                term.set_cursor_blink_mode(if cfg.text.cursor.blinks {
                    vte::CursorBlinkMode::On
                } else {
                    vte::CursorBlinkMode::Off
                });
                term.set_scrollback_lines(match cfg.text.scrollback {
                    Scrollback::Finite(num) => num as i64,
                    Scrollback::Infinite => -1,
                });
                term.set_colors(
                    Some(&match cfg.text.color {
                        TextColor::Black => cfg.palette.black.into(),
                        TextColor::White => cfg.palette.white.into(),
                        TextColor::Custom(color) => color.into(),
                    }),
                    None,
                    &[
                        &cfg.palette.black.into(),
                        &cfg.palette.red.into(),
                        &cfg.palette.green.into(),
                        &cfg.palette.yellow.into(),
                        &cfg.palette.blue.into(),
                        &cfg.palette.magenta.into(),
                        &cfg.palette.cyan.into(),
                        &cfg.palette.light_grey.into(),
                        &cfg.palette.dark_grey.into(),
                        &cfg.palette.brown.into(),
                        &cfg.palette.light_red.into(),
                        &cfg.palette.light_green.into(),
                        &cfg.palette.light_blue.into(),
                        &cfg.palette.light_magenta.into(),
                        &cfg.palette.light_cyan.into(),
                        &cfg.palette.white.into(),
                    ],
                );
                match cfg.background {
                    Background::SolidColor(color) => {
                        term.set_color_background(&match color {
                            BackgroundColor::Black => cfg.palette.black.into(),
                            BackgroundColor::White => cfg.palette.white.into(),
                            BackgroundColor::Custom(color) => color.into(),
                        });
                    }
                    _ => {}
                };
                match &cfg.text.font {
                    crate::config::Font::System => term.set_font(None),
                    crate::config::Font::Custom(font) => {
                        term.set_font(Some(&(font.into())));
                    }
                }
            }
        }
    }
}
