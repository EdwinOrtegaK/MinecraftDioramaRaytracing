use nalgebra::Vector3;
use crate::color::Color;

#[derive(Clone)]
pub struct Light {
    pub position: Vector3<f32>,  // Posición de la luz en el espacio 3D
    pub color: Color,            // Color de la luz
    pub intensity: f32,          // Intensidad de la luz
    pub radius: f32,             // Radio de la luz
}

impl Light {
    pub fn new(position: Vector3<f32>, color: Color, intensity: f32, radius: f32) -> Self {
        Light {
            position,
            color,
            intensity,
            radius,
        }
    }
}
