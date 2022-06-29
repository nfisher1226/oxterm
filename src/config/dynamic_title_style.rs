use {serde::{Deserialize, Serialize}, std::fmt};

#[derive(Default, Deserialize, Serialize)]
pub enum DynamicTitleStyle {
    ReplacesTitle,
    BeforeTitle,
    #[default]
    AfterTitle,
    NotDisplayed,
}

impl fmt::Display for DynamicTitleStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", match self {
            Self::ReplacesTitle => "Replaces Title",
            Self::BeforeTitle => "Before Title",
            Self::AfterTitle => "After Title",
            Self::NotDisplayed => "Not Displayed",
        })
    }
}
