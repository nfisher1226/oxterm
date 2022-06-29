use {serde::{Deserialize, Serialize}, std::fmt};

#[derive(Default, Deserialize, Serialize)]
pub enum Cursor {
    #[default]
    Block,
    Ibeam,
    Underline,
}

impl fmt::Display for Cursor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", match self {
            Self::Block => "Block",
            Self::Ibeam => "Ibeam",
            Self::Underline => "Underline",
        })
    }
}
