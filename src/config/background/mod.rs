use {
    super::Color,
    crate::CONFIG,
    serde::{Deserialize, Serialize},
    std::{fmt::Display, string::ToString},
};

pub mod gradient;
pub mod image;
pub use {gradient::Gradient, image::Image};

#[derive(Clone, Copy, Default, Debug, Deserialize, Serialize)]
pub enum BackgroundColor {
    #[default]
    Black,
    White,
    Custom(Color),
}

impl From<BackgroundColor> for Color {
    fn from(color: BackgroundColor) -> Self {
        let palette = &CONFIG.try_lock().unwrap().palette;
        match color {
            BackgroundColor::Black => palette.black,
            BackgroundColor::White => palette.white,
            BackgroundColor::Custom(c) => c,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Background {
    SolidColor(BackgroundColor),
    Image(Image),
    Gradient(Gradient),
}

impl Default for Background {
    fn default() -> Self {
        Self::SolidColor(BackgroundColor::default())
    }
}

pub trait AsCss<T> 
where T: Display + ToString {
    fn as_css(&self) -> T;
}

impl AsCss<std::string::String> for Color {
    fn as_css(&self) -> std::string::String {
        format!(
            ".workview stack {{\n    background-color: {};\n    background-size: 100% 100%;\n}}",
            self,
        )
    }
}
