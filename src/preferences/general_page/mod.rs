mod imp;

use {
    crate::Values,
    gtk::{
        glib::{self, GString, Object},
        prelude::*,
        subclass::prelude::*,
    },
};

use crate::config::{DynamicTitleStyle, General, TabPosition};

glib::wrapper! {
    pub struct GeneralPage(ObjectSubclass<imp::GeneralPage>)
        @extends gtk::Grid, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
            gtk::Orientable;
}

impl Default for GeneralPage {
    fn default() -> Self {
        Self::new()
    }
}

impl GeneralPage {
    #[must_use]
    pub fn new() -> Self {
        Object::new(&[]).expect("Cannot create general page")
    }

    pub fn initial_title(&self) -> String {
        self.imp().initial_title.text().to_string()
    }

    pub fn set_initial_title(&self, title: &str) {
        self.imp().initial_title.set_text(title);
    }

    pub fn title_style(&self) -> DynamicTitleStyle {
        self.imp()
            .dynamic_title
            .active_id()
            .unwrap_or(GString::from(""))
            .as_str()
            .parse()
            .unwrap_or_default()
    }

    pub fn set_title_style(&self, style: &DynamicTitleStyle) {
        self.imp().dynamic_title.set_active_id(Some(match style {
            DynamicTitleStyle::ReplacesTitle => "replaces_title",
            DynamicTitleStyle::BeforeTitle => "before_title",
            DynamicTitleStyle::AfterTitle => "after_title",
            DynamicTitleStyle::NotDisplayed => "not_displayed",
        }));
    }

    pub fn tab_position(&self) -> TabPosition {
        self.imp()
            .tab_position
            .active_id()
            .unwrap_or(GString::from(""))
            .as_str()
            .parse()
            .unwrap_or_default()
    }

    pub fn set_tab_position(&self, pos: &TabPosition) {
        self.imp().tab_position.set_active_id(Some(match pos {
            TabPosition::Top => "top",
            TabPosition::Bottom => "bottom",
            TabPosition::Left => "left",
            TabPosition::Right => "right",
        }));
    }

    pub fn wide_handles(&self) -> bool {
        self.imp().wide_handles.is_active()
    }

    pub fn set_wide_handles(&self, active: bool) {
        self.imp().wide_handles.set_active(active);
    }

    pub fn custom_command(&self) -> Option<String> {
        let imp = self.imp();
        if imp.custom_command_checkbutton.is_active() {
            Some(imp.custom_command.text().to_string())
        } else {
            None
        }
    }

    pub fn set_custom_command(&self, cmd: Option<String>) {
        let imp = self.imp();
        match cmd {
            Some(c) => {
                imp.custom_command_checkbutton.set_active(true);
                imp.custom_command.set_text(&c);
                imp.custom_command.set_sensitive(true);
            }
            None => {
                imp.custom_command_checkbutton.set_active(false);
                imp.custom_command.set_text("");
                imp.custom_command.set_sensitive(false);
            }
        }
    }
}

impl Values<General> for GeneralPage {
    fn values(&self) -> General {
        General {
            initial_title: self.initial_title(),
            title_style: self.title_style(),
            tab_position: self.tab_position(),
            wide_handles: self.wide_handles(),
            custom_command: self.custom_command(),
        }
    }

    fn set_values(&self, gen: &General) {
        self.set_initial_title(&gen.initial_title);
        self.set_title_style(&gen.title_style);
        self.set_tab_position(&gen.tab_position);
        self.set_wide_handles(gen.wide_handles);
        self.set_custom_command(gen.custom_command.clone());
    }
}
