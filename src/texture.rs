use image::{GenericImageView, DynamicImage};
use crate::color::Color;

#[derive(Debug, Clone)]
pub struct Texture {
    pub data: Vec<Color>,
    pub width: usize,
    pub height: usize,
}


impl Texture {
    pub fn load_from_file(path: &str) -> Self {
        let img = image::open(path).expect("Failed to load texture");
        let (width, height) = img.dimensions();
        let mut data = Vec::new();

        for (_, _, pixel) in img.pixels() {
            let rgba = pixel.0;
            let color = Color::new(rgba[0], rgba[1], rgba[2]);
            data.push(color);
        }

        Texture {
            width: width as usize,
            height: height as usize,
            data,
        }
    }

    pub fn get_color(&self, u: f32, v: f32) -> Color {
        let x = (u * (self.width - 1) as f32) as usize;
        let y = (v * (self.height - 1) as f32) as usize;
        self.data[y * self.width + x]
    }
}
