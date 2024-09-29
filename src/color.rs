#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub fn scale(&self, factor: f32) -> Self {
        Color {
            r: (self.r as f32 * factor).min(255.0) as u8,
            g: (self.g as f32 * factor).min(255.0) as u8,
            b: (self.b as f32 * factor).min(255.0) as u8,
        }
    }

    pub fn to_u32(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
}
