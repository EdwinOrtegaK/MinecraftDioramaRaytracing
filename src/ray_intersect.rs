use nalgebra::Vector3;
use crate::color::Color;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse: Color,  // Color difuso del material
    pub specular: f32,   // Coeficiente especular (controla la dureza del reflejo especular)
    pub albedo: [f32; 3], // Albedo: difuso y especular
}

impl Material {
    pub fn new(diffuse: Color, specular: f32, albedo: [f32; 3]) -> Self {
        Material {
            diffuse,
            specular,
            albedo,
        }
    }

    pub fn black() -> Self {
        Material {
            diffuse: Color::new(0, 0, 0),
            specular: 0.0,
            albedo: [0.0, 0.0, 0.0],
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Intersect {
    pub point: Vector3<f32>,  // Punto de intersección
    pub normal: Vector3<f32>, // Normal en el punto de intersección
    pub distance: f32,        // Distancia desde el origen del rayo
    pub is_intersecting: bool, // Indica si hay una intersección
    pub material: Material,   // Material del objeto en el punto de intersección
}

impl Intersect {
    pub fn new(point: Vector3<f32>, normal: Vector3<f32>, distance: f32, material: Material) -> Self {
        Intersect {
            point,
            normal,
            distance,
            is_intersecting: true,
            material,
        }
    }

    pub fn empty() -> Self {
        Intersect {
            point: Vector3::zeros(),
            normal: Vector3::zeros(),
            distance: 0.0,
            is_intersecting: false,
            material: Material::black(),
        }
    }
}

pub trait RayIntersect {
    fn ray_intersect(&self, ray_origin: &Vector3<f32>, ray_direction: &Vector3<f32>) -> Intersect;
}

pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32,
    pub material: Material,
}

impl RayIntersect for Sphere {
    fn ray_intersect(&self, ray_origin: &Vector3<f32>, ray_direction: &Vector3<f32>) -> Intersect {
        let oc = ray_origin - self.center;

        let a = ray_direction.dot(ray_direction);
        let b = 2.0 * oc.dot(ray_direction);
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let distance = (-b - discriminant.sqrt()) / (2.0 * a);
            if distance > 0.0 {
                let point = ray_origin + ray_direction * distance;
                let normal = (point - self.center).normalize();
                return Intersect::new(point, normal, distance, self.material);
            }
        }
        Intersect::empty()
    }
}
