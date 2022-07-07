use {
    gtk::PositionType,
    serde::{Deserialize, Serialize},
    std::fmt,
};

#[derive(Clone, Default, Deserialize, Serialize)]
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
