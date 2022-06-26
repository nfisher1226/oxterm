pub mod actions;
pub mod cli;
pub mod config;
pub mod keys;
pub mod ox_window;
pub mod tab;
pub mod tab_label;

use {gtk::prelude::*, std::rc::Rc};
pub use {keys::Keys, ox_window::OxWindow, tab::Tab, tab_label::TabLabel};

pub fn build_ui(app: &gtk::Application) -> Rc<OxWindow> {
    let window = Rc::new(OxWindow::new(app));
    actions::add(&window, app);
    let _tab = window.new_tab();
    window.present();
    window
}
