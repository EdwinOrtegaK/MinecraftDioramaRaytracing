use nalgebra::Vector3;
use std::f32::consts::PI;

pub struct Camera {  // Hacer pública la estructura Camera
    pub eye: Vector3<f32>,    // Posición de la cámara
    pub center: Vector3<f32>, // Punto que la cámara mira
    pub up: Vector3<f32>,     // Vector 'arriba'
}

impl Camera {
    // Constructor de la cámara
    pub fn new(eye: Vector3<f32>, center: Vector3<f32>, up: Vector3<f32>) -> Camera {
        Camera { eye, center, up }
    }

    // Método para hacer un cambio de base de los vectores
    pub fn basis_change(&self, vector: &Vector3<f32>) -> Vector3<f32> {
        let forward = (self.center - self.eye).normalize();
        let right = forward.cross(&self.up).normalize();
        let up = right.cross(&forward).normalize();

        vector.x * right + vector.y * up - vector.z * forward
    }

    // Método para orbitar la cámara usando yaw y pitch
    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        let radius_vector = self.eye - self.center;
        let radius = radius_vector.magnitude();

        let current_yaw = radius_vector.z.atan2(radius_vector.x);
        let radius_xz = (radius_vector.x.powi(2) + radius_vector.z.powi(2)).sqrt();
        let current_pitch = (-radius_vector.y).atan2(radius_xz);

        let new_yaw = (current_yaw + delta_yaw) % (2.0 * PI);
        let new_pitch = (current_pitch + delta_pitch).clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);

        self.eye = self.center + Vector3::new(
            radius * new_yaw.cos() * new_pitch.cos(),
            -radius * new_pitch.sin(),
            radius * new_yaw.sin() * new_pitch.cos(),
        );
    }
}
