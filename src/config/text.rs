use {
    super::{Color, Cursor, Font, Scrollback},
    serde::{Deserialize, Serialize},
};

#[derive(Clone, Deserialize, Serialize)]
pub enum TextColor {
    Black,
    White,
    Custom(Color),
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Text {
    pub cursor: Cursor,
    pub scrollback: Scrollback,
    pub font: Font,
    pub color: TextColor,
}
