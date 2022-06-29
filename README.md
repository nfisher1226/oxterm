## Summary
Terminal emulator in Rust and Gtk4. Currently in an early stage of development.
OxTerm is a possible spiritual successor to [Zterm](https://codeberg.org/jeang3nie/zterm)
in that it is planned to be relatively simple, with a minimal graphical interface,
yet have a few nice power user features such as split panes as well as some nice
theming options. It is also an acknowledgement that using a pre-1.0 language
such as Zig comes with some major challenges. Finally, there is a lot that was
learned both in impllementing Zterm and in other projects since, and the code in
Zterm is definitely sub-optimal in a number of areas which lead to challenges in
the future development of the codebase.
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
### Other Unix/Linux
A lot of Linux distros are significantly behind in packaging Gtk+ version 4 and
will not have a new enough version in their official repos to build OxTerm. It
is even less likely that you will find a package for vte compiled for gtk4. It
is still possible to install OxTerm on these systems by compiling the required
dependencies from source. This situation is expected to improve rapidly over the
next year or so as Gnome transitions more completely from Gtk+ version 3 to Gtk+
version 4.

FreeBSD has a compatible package for Gtk+ version 4, but not for Vte.
