use {
    super::Color,
    crate::CONFIG,
    serde::{Deserialize, Serialize},
    std::{fmt::{self, Display}, string::ToString},
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
where
    T: Display + ToString,
{
    fn as_css(&self) -> T;
}

impl AsCss<std::string::String> for Background {
    fn as_css(&self) -> std::string::String {
        match self {
            Self::SolidColor(bc) => {
                format!(".workview stack {{\n    {bc}\n    background-size: 100% 100%;\n}}")
            }
            Self::Image(img) => format!(
                ".workview stack {{\n    background-image: url(\"{}\");\n{}}}",
                img.file.display(),
                img.style.as_css(),
            ),
            Self::Gradient(g) => g.as_css(),
        }
    }
}

impl fmt::Display for BackgroundColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Color::from(*self))
    }
}
