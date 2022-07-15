use {
    serde::{Deserialize, Serialize},
    std::{error::Error, fmt, str::FromStr},
};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum DynamicTitleStyle {
    ReplacesTitle,
    BeforeTitle,
    #[default]
    AfterTitle,
    NotDisplayed,
}

impl fmt::Display for DynamicTitleStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Self::ReplacesTitle => "Replaces Title",
                Self::BeforeTitle => "Before Title",
                Self::AfterTitle => "After Title",
                Self::NotDisplayed => "Not Displayed",
            }
        )
    }
}

#[derive(Debug)]
pub struct ParseTitleStyleError;

impl fmt::Display for ParseTitleStyleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error parsing dynamic title style")
    }
}

impl Error for ParseTitleStyleError {}

impl FromStr for DynamicTitleStyle {
    type Err = ParseTitleStyleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "replaces_title" | "Replaces Title" => Ok(Self::ReplacesTitle),
            "before_title" | "Before Title" => Ok(Self::BeforeTitle),
            "after_title" | "After Title" => Ok(Self::AfterTitle),
            "not_displayed" | "Not Displayed" => Ok(Self::NotDisplayed),
            _ => Err(ParseTitleStyleError),
        }
    }
}
