use nalgebra::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse: u32,  // Color representado como un entero
}

#[derive(Debug, Clone, Copy)]
pub struct Intersect {
    pub point: Vector3<f32>,  // Punto de intersección
    pub normal: Vector3<f32>, // Normal en el punto de intersección
    pub distance: f32,        // Distancia desde el origen del rayo
    pub is_intersecting: bool,  // Indica si hay una intersección
    pub material: Material,   // Material del objeto en el punto de intersección
}

impl Intersect {
    // Constructor para un punto de intersección válido
    pub fn new(point: Vector3<f32>, normal: Vector3<f32>, distance: f32, material: Material) -> Self {
        Intersect {
            point,
            normal,
            distance,
            is_intersecting: true,
            material,
        }
    }

    // Constructor para un estado vacío (sin intersección)
    pub fn empty() -> Self {
        Intersect {
            point: Vector3::zeros(),
            normal: Vector3::zeros(),
            distance: 0.0,
            is_intersecting: false,
            material: Material { diffuse: 0x000000 },  // Color negro por defecto
        }
    }
}

// Trait para la intersección de rayos
pub trait RayIntersect {
    fn ray_intersect(&self, ray_origin: &Vector3<f32>, ray_direction: &Vector3<f32>) -> Intersect;
}