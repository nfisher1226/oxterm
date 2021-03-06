use std::{fs, io::BufWriter};

use {
    super::ConfigError,
    rgba_simple::{PrimaryColor, RGBA},
    ron::ser::PrettyConfig,
    serde::{Deserialize, Serialize},
};

pub type Color = RGBA<f32>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Palette {
    pub name: String,
    pub black: Color,
    pub red: Color,
    pub green: Color,
    pub brown: Color,
    pub blue: Color,
    pub magenta: Color,
    pub cyan: Color,
    pub light_grey: Color,
    pub dark_grey: Color,
    pub light_red: Color,
    pub light_green: Color,
    pub yellow: Color,
    pub light_blue: Color,
    pub light_magenta: Color,
    pub light_cyan: Color,
    pub white: Color,
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            name: String::from("Default"),
            black: PrimaryColor::Black.into(),
            red: RGBA::new(0.647, 0.114, 0.176, 1.0),
            green: RGBA::new(0.0, 0.666, 0.0, 1.0),
            brown: RGBA::new(0.388, 0.270, 0.173, 1.0),
            blue: RGBA::new(0.0, 0.0, 0.666, 1.0),
            magenta: RGBA::new(0.666, 0.0, 0.666, 1.0),
            cyan: RGBA::new(0.0, 0.666, 0.666, 1.0),
            light_grey: RGBA::new(0.666, 0.666, 0.666, 1.0),
            dark_grey: RGBA::new(0.333, 0.333, 0.333, 1.0),
            light_red: RGBA::new(1.0, 0.333, 0.333, 1.0),
            light_green: RGBA::new(0.333, 1.0, 0.333, 1.0),
            yellow: RGBA::new(1.0, 0.741, 0.0, 1.0),
            light_blue: RGBA::new(0.333, 0.333, 1.0, 1.0),
            light_magenta: RGBA::new(1.0, 0.333, 1.0, 1.0),
            light_cyan: RGBA::new(0.333, 1.0, 1.0, 1.0),
            white: PrimaryColor::White.into(),
        }
    }
}

impl Palette {
    pub fn new_from(&self, name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..*self
        }
    }

    pub fn load(name: &str) -> Result<Self, ConfigError> {
        let mut file = super::get_data_dir();
        file.push("palettes");
        file.push(name);
        let contents = fs::read_to_string(file)?;
        let cp = ron::de::from_str(&contents)?;
        Ok(cp)
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        let mut file = super::get_data_dir();
        file.push("palettes");
        if !file.exists() {
            fs::create_dir_all(&file)?;
        }
        file.push(&self.name.to_lowercase());
        file.set_extension("ron");
        let pcfg = PrettyConfig::new().struct_names(true).decimal_floats(true);
        let file = fs::File::create(&file)?;
        let buf = BufWriter::new(file);
        ron::ser::to_writer_pretty(buf, self, pcfg)?;
        Ok(())
    }
}

pub fn get_palette_names() -> Vec<(String, String)> {
    let mut palettes = vec![];
    if let Ok(dir) = fs::read_dir(super::get_data_dir()) {
        for file in dir.flatten() {
            if let Some(name) = file.path().file_name() {
                let name = name.to_string_lossy().to_string();
                if let Ok(palette) = Palette::load(&name) {
                    palettes.push((name, palette.name));
                }
            }
        }
    }
    palettes
}
