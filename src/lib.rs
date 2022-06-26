pub mod cli;
pub mod config;
pub mod ox_window;
pub mod tab;
pub mod tab_label;

use {gtk::prelude::*, std::rc::Rc};
pub use {tab::Tab, tab_label::TabLabel, ox_window::OxWindow};

pub fn build_ui(app: &gtk::Application) -> Rc<OxWindow> {
    let window = Rc::new(OxWindow::new(app));
    let tab = Tab::new();
    window.notebook().append_page(&tab, Some(&tab.label()));
    window.present();
    window
}
