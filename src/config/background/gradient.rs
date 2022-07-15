use {
    crate::config::Color,
    rgba_simple::{PrimaryColor, RGBA},
    serde::{Deserialize, Serialize},
    std::{cmp::Ordering, error::Error, fmt, str::FromStr},
};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum VerticalPlacement {
    #[default]
    Top,
    Center,
    Bottom,
}

#[derive(Debug)]
pub struct ParsePlacementError;

impl fmt::Display for ParsePlacementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error parsing placement")
    }
}

impl Error for ParsePlacementError {}

impl fmt::Display for VerticalPlacement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Top => "Top",
                Self::Center => "Center",
                Self::Bottom => "Bottom",
            }
        )
    }
}

impl FromStr for VerticalPlacement {
    type Err = ParsePlacementError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Top" | "top" => Ok(Self::Top),
            "Center" | "center" => Ok(Self::Center),
            "Bottom" | "bottom" => Ok(Self::Bottom),
            _ => Err(ParsePlacementError),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum HorizontalPlacement {
    #[default]
    Left,
    Center,
    Right,
}

impl fmt::Display for HorizontalPlacement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Left => "Left",
                Self::Center => "Center",
                Self::Right => "Right",
            }
        )
    }
}

impl FromStr for HorizontalPlacement {
    type Err = ParsePlacementError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Left" | "left" => Ok(Self::Left),
            "Center" | "center" => Ok(Self::Center),
            "Right" | "right" => Ok(Self::Right),
            _ => Err(ParsePlacementError),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Placement {
    pub vertical: VerticalPlacement,
    pub horizontal: HorizontalPlacement,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Direction {
    Angle(f64),
    Edge(Placement),
}

impl Default for Direction {
    fn default() -> Self {
        Self::Edge(Placement::default())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Kind {
    Linear(Direction),
    Radial(Placement),
    Elliptical(Placement),
}

impl Default for Kind {
    fn default() -> Self {
        Self::Linear(Direction::default())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Stop {
    pub color: Color,
    pub position: f64,
}

impl Stop {
    pub const MIN_POSITION: f64 = 0.0;
    pub const MAX_POSITION: f64 = 100.0;

    pub fn new(color: Color, position: f64) -> Self {
        Self { color, position }
    }
}

impl PartialOrd for Stop {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.position.partial_cmp(&other.position)
    }
}

impl PartialEq for Stop {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Gradient {
    pub kind: Kind,
    pub stops: Vec<Stop>,
}

impl Default for Gradient {
    fn default() -> Self {
        Self {
            kind: Kind::default(),
            stops: vec![
                Stop {
                    color: PrimaryColor::Black.into(),
                    position: 0.0,
                },
                Stop {
                    color: RGBA::new(0.64, 0.64, 0.64, 1.0),
                    position: 100.0,
                },
            ],
        }
    }
}
