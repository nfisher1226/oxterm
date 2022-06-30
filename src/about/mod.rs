use {crate::ox_window::OxWindow, gtk::prelude::*, std::process::Command};

pub fn show(window: &OxWindow) {
    let dlg = gtk::AboutDialog::builder()
        .comments("An Oxidized terminal for the Unix desktop")
        .copyright("Copyright © 2022 by Nathan Fisher")
        .license_type(gtk::License::Gpl30)
        .logo_icon_name("oxterm")
        .program_name("OxTerm")
        .system_information(&format!(
            "Gtk+ version: {}\nVte version: {}\nSystem: {}",
            gtk4_version(),
            vte4_version(),
            system(),
        ))
        .version(env!("CARGO_PKG_VERSION"))
        .website("https://codeberg.org/oxterm")
        .destroy_with_parent(true)
        .modal(true)
        .transient_for(window)
        .build();
    dlg.show();
}

fn gtk4_version() -> String {
    let cmd = Command::new("pkg-config")
        .args(["--modversion", "gtk4"])
        .output();
    match cmd {
        Ok(c) => String::from_utf8_lossy(&c.stdout).to_string(),
        Err(_) => String::from("unknown"),
    }
}

fn vte4_version() -> String {
    let cmd = Command::new("pkg-config")
        .args(["--modversion", "vte-2.91-gtk4"])
        .output();
    match cmd {
        Ok(c) => String::from_utf8_lossy(&c.stdout).to_string(),
        Err(_) => String::from("unknown"),
    }
}

fn system() -> String {
    let cmd = Command::new("uname").arg("-a").output();
    match cmd {
        Ok(c) => String::from_utf8_lossy(&c.stdout).to_string(),
        Err(_) => String::from("unknown"),
    }
}
