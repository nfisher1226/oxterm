use {
    serde::{Deserialize, Serialize},
    std::{error::Error, fmt, path::PathBuf, str::FromStr},
};

#[derive(Default, Deserialize, Serialize)]
pub enum Style {
    Tiled,
    Centered,
    #[default]
    Scaled,
    Stretched,
}

#[derive(Deserialize, Serialize)]
pub struct Image {
    file: PathBuf,
    style: Style,
}

#[derive(Debug)]
pub struct ParseImageStyleError;

impl fmt::Display for ParseImageStyleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error parsing image style")
    }
}

impl Error for ParseImageStyleError {}

impl FromStr for Style {
    type Err = ParseImageStyleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tiled" | "Tiled" => Ok(Self::Tiled),
            "centered" | "Centered" => Ok(Self::Centered),
            "scaled" | "Scaled" => Ok(Self::Scaled),
            "stretched" | "Stretched" => Ok(Self::Stretched),
            _ => Err(ParseImageStyleError),
        }
    }
}
