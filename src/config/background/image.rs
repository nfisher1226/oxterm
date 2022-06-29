use {serde::{Deserialize, Serialize}, std::path::PathBuf};

#[derive(Default, Deserialize, Serialize)]
pub enum Style {
    Tiled,
    Centered,
    #[default]
    Scaled,
    Stretched,
}

#[derive(Deserialize, Serialize)]
pub struct Image {
    file: PathBuf,
    style: Style,
}
