use {
    crate::config::Color,
    rgba_simple::{PrimaryColor, RGBA},
    serde::{Deserialize, Serialize},
    std::cmp::Ordering,
};

#[derive(Default, Deserialize, Serialize)]
pub enum VerticalPlacement {
    #[default]
    Top,
    Center,
    Bottom,
}

#[derive(Default, Deserialize, Serialize)]
pub enum HorizontalPlacement {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(Default, Deserialize, Serialize)]
pub struct Placement {
    pub vertical: VerticalPlacement,
    pub horizontal: HorizontalPlacement,
}

#[derive(Deserialize, Serialize)]
pub enum Direction {
    Angle(f64),
    Edge(Placement),
}

impl Default for Direction {
    fn default() -> Self {
        Self::Edge(Placement::default())
    }
}

#[derive(Deserialize, Serialize)]
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

#[derive(Deserialize, Serialize)]
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

#[derive(Deserialize, Serialize)]
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
