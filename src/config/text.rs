use {
    super::{Color, Cursor, Font, Scrollback},
    serde::{Deserialize, Serialize},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum TextColor {
    Black,
    White,
    Custom(Color),
}

impl Default for TextColor {
    fn default() -> Self {
        Self::White
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Text {
    pub cursor: Cursor,
    pub scrollback: Scrollback,
    pub font: Font,
    pub color: TextColor,
}
