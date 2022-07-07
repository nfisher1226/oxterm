use {
    crate::preferences::GeneralPage, 
    serde::{Deserialize, Serialize},
    super::{DynamicTitleStyle, TabPosition},
};

#[derive(Clone, Deserialize, Serialize)]
pub struct General {
    pub initial_title: String,
    pub title_style: DynamicTitleStyle,
    pub custom_command: Option<String>,
    pub tab_position: TabPosition,
}

impl Default for General {
    fn default() -> Self {
        Self {
            initial_title: String::from(env!("CARGO_PKG_NAME")),
            title_style: DynamicTitleStyle::default(),
            custom_command: None,
            tab_position: TabPosition::default(),
        }
    }
}

impl From<&GeneralPage> for General {
    fn from(page: &GeneralPage) -> Self {
        Self {
            initial_title: page.initial_title(),
            title_style: page.title_style(),
            custom_command: page.custom_command(),
            tab_position: page.tab_position(),
        }
    }
}
