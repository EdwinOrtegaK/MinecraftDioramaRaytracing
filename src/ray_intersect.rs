use nalgebra::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse: u32,  // Color representado como un entero
}

#[derive(Debug, Clone, Copy)]
pub struct Intersect {
    pub distance: f32,
    pub is_intersecting: bool,
    pub material: Material,
}

impl Intersect {
    pub fn new(distance: f32, material: Material) -> Self {
        Intersect {
            distance,
            is_intersecting: true,
            material,
        }
    }

    pub fn empty() -> Self {
        Intersect {
            distance: 0.0,
            is_intersecting: false,
            material: Material { diffuse: 0x000000 },  // Color negro por defecto
        }
    }
}

pub trait RayIntersect {
    fn ray_intersect(&self, ray_origin: &Vector3<f32>, ray_direction: &Vector3<f32>) -> Intersect;
}
