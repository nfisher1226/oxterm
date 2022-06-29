use {rgba_simple::{PrimaryColor, RGBA}, serde::{Deserialize, Serialize}};

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
    vertical: VerticalPlacement,
    horizontal: HorizontalPlacement,
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
    color: RGBA<f32>,
    position: f64,
}

#[derive(Deserialize, Serialize)]
pub struct Gradient {
    kind: Kind,
    start: Stop,
    mid: Vec<Stop>,
    end: Stop,
}

impl Default for Gradient {
    fn default() -> Self {
        Self {
            kind: Kind::default(),
            start: Stop {
                color: PrimaryColor::Black.into(),
                position: 0.0,
            },
            mid: vec![],
            end: Stop {
                color: RGBA::new(0.64, 0.64, 0.64, 1.0),
                position: 100.0,
            },
        }
    }
}
