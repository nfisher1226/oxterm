use {
    gtk::PositionType,
    serde::{Deserialize, Serialize},
    std::{error::Error, fmt, str::FromStr},
};

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub enum TabPosition {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

impl fmt::Display for TabPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Self::Top => "Top",
                Self::Bottom => "Bottom",
                Self::Left => "Left",
                Self::Right => "Right",
            }
        )
    }
}

impl From<TabPosition> for PositionType {
    fn from(pos: TabPosition) -> Self {
        match pos {
            TabPosition::Top => Self::Top,
            TabPosition::Bottom => Self::Bottom,
            TabPosition::Left => Self::Left,
            TabPosition::Right => Self::Right,
        }
    }
}

#[derive(Debug)]
pub struct ParseTabPositionError;

impl fmt::Display for ParseTabPositionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error parsing PositionType")
    }
}

impl Error for ParseTabPositionError {}

impl FromStr for TabPosition {
    type Err = ParseTabPositionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "top" | "Top" => Ok(Self::Top),
            "bottom" | "Bottom" => Ok(Self::Bottom),
            "left" | "Left" => Ok(Self::Left),
            "right" | "Right" => Ok(Self::Right),
            _ => Err(ParseTabPositionError),
        }
    }
}
