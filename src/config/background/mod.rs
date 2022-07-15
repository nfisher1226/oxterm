use {
    super::Color,
    serde::{Deserialize, Serialize},
};

pub mod gradient;
pub mod image;
pub use {gradient::Gradient, image::Image};

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub enum BackgroundColor {
    #[default]
    Black,
    White,
    Custom(Color),
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
