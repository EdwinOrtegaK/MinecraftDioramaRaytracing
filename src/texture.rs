use image::{GenericImageView, DynamicImage};
use crate::color::Color;

#[derive(Debug, Clone, PartialEq)]
pub struct Texture {
    pub data: Vec<Color>,
    pub width: usize,
    pub height: usize,
}

impl Color {
    pub fn r(&self) -> u8 {
        self.r
    }
    pub fn g(&self) -> u8 {
        self.g
    }
    pub fn b(&self) -> u8 {
        self.b
    }
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

    // Método para obtener el ancho de la textura
    pub fn width(&self) -> usize {
        self.width
    }

    // Método para obtener la altura de la textura
    pub fn height(&self) -> usize {
        self.height
    }

    // Método para obtener el color de un píxel en una posición (x, y)
    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        self.data[y * self.width + x]
    }

    pub fn get_color(&self, u: f32, v: f32) -> Color {
        let x = (u * (self.width - 1) as f32) as usize;
        let y = (v * (self.height - 1) as f32) as usize;
        self.get_pixel(x, y)
    }
}
