#![warn(clippy::all, clippy::pedantic)]
#![doc = include_str!("../README.md")]

use std::{io, ffi::OsString, os::unix::prelude::OsStringExt};
mod about;
pub mod actions;
pub mod cli;
pub mod config;
mod font;
mod keys;
mod ox_window;
mod preferences;
mod tab;
mod tab_label;

use {
    gtk::{
        glib::{self, clone},
        prelude::*,
    },
    once_cell::sync::Lazy,
    std::{ffi::CStr, rc::Rc, sync::Mutex},
};

pub use {
    config::Config,
    font::{Font, ParseFontError},
    keys::Keys,
    ox_window::OxWindow,
    preferences::PreferencesDialog,
    tab::Tab,
    tab_label::TabLabel,
};

static SHELL: Lazy<&'static str> = Lazy::new(|| {
    let shell = unsafe { CStr::from_ptr(vte::ffi::vte_get_user_shell()) };
    shell.to_str().unwrap_or("/bin/sh")
});

static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| Mutex::new(Config::load().unwrap_or_default()));

pub trait Values<V> {
    fn values(&self) -> V;
    fn set_values(&self, values: &V);
}

pub fn gethostname() -> io::Result<String> {
    let size =
        unsafe { libc::sysconf(libc::_SC_HOST_NAME_MAX) as libc::size_t };
    let mut buffer = vec![0u8; size];
    let result = unsafe {
        libc::gethostname(buffer.as_mut_ptr() as *mut libc::c_char, size)
    };
    if result != 0 {
        return Err(io::Error::last_os_error());
    }
    let end = buffer
        .iter()
        .position(|&byte| byte == 0x00)
        .unwrap_or_else(|| buffer.len());
    buffer.resize(end, 0x00);
    Ok(OsString::from_vec(buffer).to_string_lossy().to_string())
}

#[must_use]
pub fn build_ui(app: &gtk::Application) -> Rc<OxWindow> {
    let window = Rc::new(OxWindow::new(app));
    actions::add(&window, app);
    let _tab = window.new_tab();
    let notebook = window.notebook();
    window.apply_config();
    notebook.connect_page_removed(clone!(@weak window => move |nb,_page,_| {
        if nb.n_pages() == 0 {
            window.close();
        }
    }));
    notebook.connect_switch_page(move |_nb, tab, _num| {
        if let Ok(tab) = tab.clone().downcast::<Tab>() {
            tab.terms()
                .borrow()
                .values()
                .next()
                .map(gtk::prelude::WidgetExt::grab_focus);
        }
    });
    window.present();
    window
}
