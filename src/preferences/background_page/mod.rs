mod imp;

use {
    crate::{config::{Background, BackgroundColor, Gradient, Image, ImageStyle}, Values},
    gtk::{
        gio::File,
        glib::{self, GString, Object},
        prelude::*,
        subclass::prelude::*,
    },
    std::path::Path,
};

glib::wrapper! {
    pub struct BackgroundPage(ObjectSubclass<imp::BackgroundPage>)
        @extends gtk::Grid, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
            gtk::Orientable;
}

impl Default for BackgroundPage {
    fn default() -> Self {
        Self::new()
    }
}

impl BackgroundPage {
    #[must_use]
    pub fn new() -> Self {
        let obj: Self = Object::new(&[]).expect("Cannot create background page");
        obj
    }

    pub fn color(&self) -> BackgroundColor {
        match self.imp().color_type.active_id().unwrap_or(GString::from("")).as_str() {
            "black" => BackgroundColor::Black,
            "white" => BackgroundColor::White,
            "custom" => BackgroundColor::Custom(self.imp().color_button.rgba().into()),
            _ => BackgroundColor::default(),
        }
    }

    pub fn set_color(&self, color: &BackgroundColor) {
        match color {
            BackgroundColor::Black => self.imp().color_type.set_active_id(Some("black")),
            BackgroundColor::White => self.imp().color_type.set_active_id(Some("white")),
            BackgroundColor::Custom(color) => {
                self.imp().color_button.set_rgba(&(*color).into());
                self.imp().background_type.set_active_id(Some("solid_color"))
            }
        };
    }

    fn image_file(&self) -> Option<File> {
        self.imp().image_file.file()
    }

    pub fn set_image_file(&self, file: &Path) -> Result<(), glib::error::Error> {
        self.imp().image_file.set_file(&File::for_path(file))
    }

    fn image_style(&self) -> ImageStyle {
        if let Some(style) = self.imp().image_style.active_id() {
            style.parse().unwrap_or_default()
        } else {
            ImageStyle::default()
        }
    }

    fn set_image_style(&self, style: &ImageStyle) {
        self.imp()
            .image_style
            .set_active_id(Some(&style.to_string().to_lowercase()));
    }

    pub fn image(&self) -> Option<Image> {
        Some(Image {
            file: {
                if let Some(i) = self.image_file().map(|x| x.path()).flatten() {
                    i
                } else {
                    return None;
                }
            },
            style: self.image_style(),
        })
    }

    pub fn set_image(&self, image: &Image) -> Result<(), glib::error::Error> {
        self.set_image_file(&image.file)?;
        self.set_image_style(&image.style);
        self.imp().background_type.set_active_id(Some("image"));
        Ok(())
    }

    pub fn gradient(&self) -> Gradient {
        self.imp().gradient_editor.values()
    }

    pub fn set_gradient(&self, gradient: &Gradient) {
        self.imp().gradient_editor.set_values(&gradient);
        self.imp().background_type.set_active_id(Some("gradient"));
    }
}

impl Values<Background> for BackgroundPage {
    fn values(&self) -> Background {
        match self.imp().background_type.active_id().unwrap_or(GString::from("")).as_str() {
            "solid_color" => Background::SolidColor(self.color()),
            "image" => {
                if let Some(image) = self.image() {
                    Background::Image(image)
                } else {
                    Background::default()
                }
            },
            "gradient" => Background::Gradient(self.gradient()),
            _ => Background::default(),
        }
    }

    fn set_values(&self, background: &Background) {
        match background {
            Background::SolidColor(color) => self.set_color(color),
            Background::Image(image) => if let Err(e) = self.set_image(image) {
                eprintln!("{e}");
            },
            Background::Gradient(gradient) => self.set_gradient(gradient),
        }
    }
}
