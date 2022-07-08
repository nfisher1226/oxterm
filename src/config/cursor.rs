use {
    serde::{Deserialize, Serialize},
    std::{error::Error, fmt, str::FromStr},
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

#[derive(Debug)]
pub struct ParseCursorError;

impl fmt::Display for ParseCursorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error parsing cursor style")
    }
}

impl Error for ParseCursorError {}

impl FromStr for CursorStyle {
    type Err = ParseCursorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "block" | "Block" => Ok(Self::Block),
            "ibeam" | "Ibeam" => Ok(Self::Ibeam),
            "underline" | "Underline" => Ok(Self::Underline),
            _ => Err(ParseCursorError),
        }
    }
}
