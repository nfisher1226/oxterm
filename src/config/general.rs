use {
    super::{DynamicTitleStyle, TabPosition},
    serde::{Deserialize, Serialize},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct General {
    pub initial_title: String,
    pub title_style: DynamicTitleStyle,
    pub tab_position: TabPosition,
    pub wide_handles: bool,
    pub custom_command: Option<String>,
}

impl Default for General {
    fn default() -> Self {
        Self {
            initial_title: String::from(env!("CARGO_PKG_NAME")),
            title_style: DynamicTitleStyle::default(),
            tab_position: TabPosition::default(),
            wide_handles: false,
            custom_command: None,
        }
    }
}
