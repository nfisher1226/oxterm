use clap::{Arg, Command};

pub fn opts() -> Command<'static> {
    Command::new("oxterm")
        .about("A terminal emulator in Rust and gtk4")
        .author("The JenG3nie <jeang3nie@hitchhiker-linux.org>")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("command")
                .help("A command to run instead of an interactive shell")
                .short('e')
                .long("command")
                .takes_value(true)
        )
        .arg(
            Arg::new("title")
                .help("The title for the window")
                .short('t')
                .long("title")
                .takes_value(true)
        )
        .arg(
            Arg::new("directory")
                .help("Set the working directory")
                .short('w')
                .long("directory")
                .takes_value(true)
        )
}

