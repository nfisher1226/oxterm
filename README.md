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
## Currently Broken
- Can't close open terminals (can close entire tabs though)
- Can't open more than one additional pane from menu, only via shortcut keys
  - Instead of looking to see which terminal has focus at that exact moment, need
    to store the current terminal name as a property of Tab widget, so that it
    is retrievable when the menu has focus. Needs to be updated whenever switching
    panes. Bonus - this will allow the last focused terminal in each tab to grab
    focus when switching tabs.
