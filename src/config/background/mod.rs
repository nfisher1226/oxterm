use {
    rgba_simple::RGBA,
    serde::{Deserialize, Serialize},
};

pub mod gradient;
pub mod image;
pub use {gradient::Gradient, image::Image};

#[derive(Deserialize, Serialize)]
pub enum Background {
    SolidColor(RGBA<f32>),
    Image(Image),
    Gradient(Gradient),
}
