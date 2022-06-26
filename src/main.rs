use gtk::{
    gio::Cancellable,
    glib::{char::Char, OptionArg, OptionFlags},
    prelude::*,
};

fn main() {
    let _matches = oxterm::cli::opts().get_matches();
    let app = gtk::Application::new(
        Some("org.hitchhiker-linux.oxterm"),
        gtk::gio::ApplicationFlags::default(),
    );
    app.add_main_option(
        "command",
        Char::from(b't'),
        OptionFlags::NONE,
        OptionArg::String,
        "",
        None,
    );
    app.add_main_option(
        "directory",
        Char::from(b'w'),
        OptionFlags::NONE,
        OptionArg::String,
        "",
        None,
    );
    app.add_main_option(
        "title",
        Char::from(b't'),
        OptionFlags::NONE,
        OptionArg::String,
        "",
        None,
    );
    if let Err(e) = app.register(Some(&Cancellable::new())) {
        eprintln!("{e}");
    }
    app.connect_activate(move |app| {
        let _gui = oxterm::build_ui(app);
    });
    app.run();
}
