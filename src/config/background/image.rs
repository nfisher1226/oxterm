use {
    super::AsCss,
    serde::{Deserialize, Serialize},
    std::{error::Error, fmt, path::PathBuf, str::FromStr},
};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum Style {
    Tiled,
    Centered,
    #[default]
    Scaled,
    Stretched,
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Tiled => "Tiled",
                Self::Centered => "Centered",
                Self::Scaled => "Scaled",
                Self::Stretched => "Stretched",
            }
        )
    }
}

impl AsCss<&'static str> for Style {
    fn as_css(&self) -> &'static str {
        match self {
            Self::Tiled => "    background-repeat: repeat;\n",
            Self::Centered => {
                "    background-position: center;\n    \
                background-repeat: no-repeat;\n"
            }
            Self::Scaled => {
                "    background-size: contain;\n    \
                background-repeat: no-repeat;\n    \
                background-position: center;\n"
            }
            Self::Stretched => "    background-size: 100% 100%;\n",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Image {
    pub file: PathBuf,
    pub style: Style,
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
