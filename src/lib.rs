#![doc = include_str!("../README.md")]
pub mod actions;
pub mod cli;
pub mod config;
pub mod keys;
pub mod ox_window;
pub mod tab;
pub mod tab_label;

use {
    gtk::{
        glib::{self, clone},
        prelude::*,
    },
    once_cell::sync::Lazy,
    std::{ffi::CStr, rc::Rc},
};
pub use {keys::Keys, ox_window::OxWindow, tab::Tab, tab_label::TabLabel};

static SHELL: Lazy<&'static str> = Lazy::new(|| {
    let shell = unsafe { CStr::from_ptr(vte::ffi::vte_get_user_shell()) };
    shell.to_str().unwrap_or("/bin/sh")
});

pub fn build_ui(app: &gtk::Application) -> Rc<OxWindow> {
    let window = Rc::new(OxWindow::new(app));
    actions::add(&window, app);
    let _tab = window.new_tab();
    let notebook = window.notebook();
    notebook.connect_page_removed(clone!(@weak window => move |nb,_page,_| {
        if nb.n_pages() == 0 {
            window.close();
        }
    }));
    notebook.connect_switch_page(move |_nb, tab, _num| {
        if let Ok(tab) = tab.clone().downcast::<Tab>() {
            tab.terms().borrow().values().next().map(|x| x.grab_focus());
        }
    });
    window.present();
    window
}
