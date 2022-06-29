The **Oxized** terminal emulator

Contents
========
- [Summary](#summary)
- [Planned Features](#planned-features)
- [Building](#building)
  - [Arch Linux](#arch-linux)
  - [Other](#other-unix-or-linux-systems)
- [Keyboard Shortcuts](#keyboard-shortcuts)
- [Configuration](#configuration)
## Summary
OxTerm is a terminal emulator in Rust and Gtk4. It is Currently in an early
stage of development but usable for day to day tasks. OxTerm is a possible
spiritual successor to [Zterm](https://codeberg.org/jeang3nie/zterm) in that it
is planned to be relatively simple, with a minimal graphical interface, yet have
a few nice power user features such as split panes as well as some nice theming
options. It is also an acknowledgement that using a pre-1.0 language such as Zig
comes with some major challenges. Finally, there is a lot that was learned both
in implementing Zterm and in other projects since, and the code in Zterm is
definitely sub-optimal in a number of areas which lead to challenges in the
future development of the codebase.
## Planned Features
- [x] Tabbed interface
- [x] Flexible split tabs
- [ ] Theming
  - [ ] Fonts
  - [ ] Colors
  - [ ] Backgrounds
    - [ ] Solid colors
    - [ ] Gradients
    - [ ] Images
    - [ ] Transparency
- [x] User configurable keybindings
## Building
### Arch Linux
Make sure to install the required dependencies. Arch has an official package for
`gtk4` and an aur package for `vte4-git`. To install both using
[pikaur](https://aur.archlinux.org/packages/pikaur):
```Sh
pikaur -S gtk4 vte4-git
```
Once the dependencies are installed the program is compiled using Cargo.
```
cargo build --release
sudo install -v target/release/oxterm /usr/local/bin
```
### Other Unix or Linux systems
A lot of Linux distros are significantly behind in packaging Gtk+ version 4 and
will not have a new enough version in their official repos to build OxTerm. It
is even less likely that you will find a package for vte compiled for gtk4. It
is still possible to install OxTerm on these systems by compiling the required
dependencies from source. This situation is expected to improve rapidly over the
next year or so as Gnome transitions more completely from Gtk+ version 3 to Gtk+
version 4.

FreeBSD has a compatible package for Gtk+ version 4, but not for Vte.
## Keyboard Shortcuts
The following table gives the default keybindings. If any customization is
desired, see [configuration](#configuration)
| Shortcut | Action |
| -------- | ------ |
| Ctrl/Shift/T | New Tab |
| Ctrl/Shift/W | Close Tab |
| Ctrl/PageDown | Next tab |
| Ctrl/PageUp | Previous Tab |
| Ctrl/Shift/Enter | Split Tab Horizontally |
| Alt/Enter | Split Tab Vertically |
| Alt/[1-9] | Goto [num] Tab |
| Ctrl/Shift/N | New Window |
| Ctrl/Shift/A | Show About Info |
| Ctrl/Shift/Q | Close Window |
## Configuration
Most of the planned configuration items are not yet implemented.

Keybindings may be changed from their defaults by creating and editing the file
`~/.config/oxterm/keys.toml`.
```Yaml
# Sample keys.toml file
[keys]
new_tab = "<primary><Shift>S"
```
The first field is the action name, while the value is the key combination
consisting of one or more modifiers and a key value. If the action name is
misspelled or does not refer to a valid action it will be ignored, while if the
keybinding specified does not parse to a valid keybinding the program will fall
back to it's default for that action.
