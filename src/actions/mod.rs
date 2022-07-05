use {
    crate::{about, preferences, Keys, OxWindow},
    gtk::{
        gio::SimpleAction,
        glib::{self, clone},
        prelude::*,
    },
    std::rc::Rc,
};

const ACTIONS: [&str; 19] = [
    "new_tab",
    "close_tab",
    "next_tab",
    "prev_tab",
    "split_horizontal",
    "split_vertical",
    "tab1",
    "tab2",
    "tab3",
    "tab4",
    "tab5",
    "tab6",
    "tab7",
    "tab8",
    "tab9",
    "new_window",
    "open_prefs",
    "open_about",
    "quit",
];

#[allow(clippy::too_many_lines)]
pub fn add(window: &Rc<OxWindow>, app: &gtk::Application) {
    let keys = Keys::from_file().unwrap_or_default();
    for name in &ACTIONS {
        let action = SimpleAction::new(name, None);
        app.set_accels_for_action(&format!("win.{}", name), &[keys.get(name)]);
        window.add_action(&action);
        match *name {
            "new_tab" => {
                action.connect_activate(clone!(@strong window => move |_,_| {
                    let _tab = window.new_tab();
                }));
            }
            "close_tab" => {
                action.connect_activate(clone!(@weak window => move |_,_| {
                    window.close_current_tab();
                }));
            }
            "next_tab" => {
                action.connect_activate(clone!(@weak window => move |_,_| {
                    window.next_tab();
                }));
            }
            "prev_tab" => {
                action.connect_activate(clone!(@weak window => move |_,_| {
                    window.prev_tab();
                }));
            }
            "split_horizontal" => {
                action.connect_activate(clone!(@weak window => move |_,_| {
                    if let Some(tab) = window.current_tab() {
                        tab.split(Some(gtk::Orientation::Horizontal));
                    }
                }));
            }
            "split_vertical" => {
                action.connect_activate(clone!(@weak window => move |_,_| {
                    if let Some(tab) = window.current_tab() {
                        tab.split(Some(gtk::Orientation::Vertical));
                    }
                }));
            }
            "tab1" => {
                action.connect_activate(clone!(@weak window => move |_,_| {
                    window.notebook().set_page(0);
                }));
            }
            "tab2" => {
                action.connect_activate(clone!(@weak window => move |_,_| {
                    window.notebook().set_page(1);
                }));
            }
            "tab3" => {
                action.connect_activate(clone!(@weak window => move |_,_| {
                    window.notebook().set_page(2);
                }));
            }
            "tab4" => {
                action.connect_activate(clone!(@weak window => move |_,_| {
                    window.notebook().set_page(3);
                }));
            }
            "tab5" => {
                action.connect_activate(clone!(@weak window => move |_,_| {
                    window.notebook().set_page(4);
                }));
            }
            "tab6" => {
                action.connect_activate(clone!(@weak window => move |_,_| {
                    window.notebook().set_page(5);
                }));
            }
            "tab7" => {
                action.connect_activate(clone!(@weak window => move |_,_| {
                    window.notebook().set_page(6);
                }));
            }
            "tab8" => {
                action.connect_activate(clone!(@weak window => move |_,_| {
                    window.notebook().set_page(7);
                }));
            }
            "tab9" => {
                action.connect_activate(clone!(@weak window => move |_,_| {
                    window.notebook().set_page(8);
                }));
            }
            "new_window" => {
                action.connect_activate(clone!(@strong app => move |_,_| {
                    let _window = crate::build_ui(&app);
                }));
            }
            "open_prefs" => {
                action.connect_activate(clone!(@weak window => move |_,_| {
                    preferences::run(&window);
                }));
            }
            "open_about" => {
                action.connect_activate(clone!(@weak window => move |_,_| {
                    about::show(&window);
                }));
            }
            "quit" => {
                action.connect_activate(clone!(@weak window => move |_,_| {
                    window.close();
                }));
            }
            _ => {}
        }
    }
}
