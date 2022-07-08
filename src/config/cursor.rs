use {
    serde::{Deserialize, Serialize},
    std::fmt,
};

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Cursor {
    pub style: CursorStyle,
    pub blinks: bool,
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub enum CursorStyle {
    #[default]
    Block,
    Ibeam,
    Underline,
}

impl fmt::Display for CursorStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Self::Block => "Block",
                Self::Ibeam => "Ibeam",
                Self::Underline => "Underline",
            }
        )
    }
}

impl From<&str> for CursorStyle {
    fn from(value: &str) -> Self {
        match value {
            "Block" | "block" => Self::Block,
            "Ibeam" | "ibeam" => Self::Ibeam,
            "Underline" | "underline" => Self::Underline,
            _ => Self::default(),
        }
    }
}
