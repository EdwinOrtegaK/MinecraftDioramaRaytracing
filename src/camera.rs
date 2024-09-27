use nalgebra::Vector3;

pub struct Camera {
    pub eye: Vector3<f32>,    // Posición de la cámara
    pub center: Vector3<f32>, // Punto en el que la cámara está mirando
    pub up: Vector3<f32>,     // Vector 'arriba'
}

impl Camera {
    pub fn new(eye: Vector3<f32>, center: Vector3<f32>, up: Vector3<f32>) -> Camera {
        Camera { eye, center, up }
    }

    pub fn basis_change(&self, vector: &Vector3<f32>) -> Vector3<f32> {
        let forward = (self.center - self.eye).normalize();
        let right = forward.cross(&self.up).normalize();
        let up = right.cross(&forward).normalize();

        // Convertimos el vector a las coordenadas de la cámara
        vector.x * right + vector.y * up - vector.z * forward
    }
}
