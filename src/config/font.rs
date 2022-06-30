use {
    gtk::pango::FontDescription,
    serde::{Deserialize, Serialize},
    std::fmt,
};

#[derive(Default, Deserialize, Serialize)]
pub enum Font {
    #[default]
    System,
    Custom(crate::font::Font),
}

impl fmt::Display for Font {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::System => write!(f, "System"),
            Self::Custom(font) => write!(f, "{}", FontDescription::from(font)),
        }
    }
}
