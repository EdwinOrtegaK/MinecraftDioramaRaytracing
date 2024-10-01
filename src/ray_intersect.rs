use nalgebra::Vector3;
use crate::color::Color;
use crate::texture::Texture;
use std::any::Any;

#[derive(Debug, Clone)]
pub struct Material {
    pub diffuse: Color,
    pub specular: f32,
    pub albedo: [f32; 4],
    pub refractive_index: f32,
    pub has_texture: bool,    
    pub texture: Option<Texture>,
}

impl Material {
    pub fn new(
        diffuse: Color, 
        specular: f32, 
        albedo: [f32; 4], 
        refractive_index: f32, 
        has_texture: bool, 
        texture: Option<Texture>  // Pasamos la textura opcional
    ) -> Self {
        Material {
            diffuse,
            specular,
            albedo,
            refractive_index,
            has_texture,
            texture,
        }
    }

    pub fn black() -> Self {
        Material {
            diffuse: Color::new(0, 0, 0),
            specular: 0.0,
            albedo: [0.0, 0.0, 0.0, 0.0],
            refractive_index: 1.0,
            has_texture: false,  // Sin textura
            texture: None,       // No hay textura
        }
    }

    // Función para obtener el color difuso, ya sea de una textura o del color base del material
    pub fn get_diffuse_color(&self, u: f32, v: f32) -> Color {
        if self.has_texture {
            // Obtener el color de la textura usando las coordenadas UV
            self.texture.as_ref().unwrap().get_color(u, v)
        } else {
            // Retornar el color difuso base
            self.diffuse
        }
    }
}

pub struct Intersect {
    pub point: Vector3<f32>,  // Punto de intersección
    pub normal: Vector3<f32>, // Normal en el punto de intersección
    pub distance: f32,        // Distancia desde el origen del rayo
    pub is_intersecting: bool, // Indica si hay una intersección
    pub material: Material,   // Material del objeto en el punto de intersección
    pub u: f32,               // Coordenada U para texturas
    pub v: f32,               // Coordenada V para texturas
}

impl Intersect {
    pub fn new(point: Vector3<f32>, normal: Vector3<f32>, distance: f32, material: Material, u: f32, v: f32) -> Self {
        Intersect {
            point,
            normal,
            distance,
            is_intersecting: true,
            material,
            u,
            v,
        }
    }

    pub fn empty() -> Self {
        Intersect {
            point: Vector3::zeros(),
            normal: Vector3::zeros(),
            distance: 0.0,
            is_intersecting: false,
            material: Material::black(),
            u: 0.0,
            v: 0.0,
        }
    }
}

pub trait RayIntersect: Any {
    fn ray_intersect(&self, ray_origin: &Vector3<f32>, ray_direction: &Vector3<f32>) -> Intersect;
    fn get_uv(&self, point: &Vector3<f32>) -> (f32, f32);
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    // Función para calcular las coordenadas UV en una esfera
    fn get_uv(&self, point: &nalgebra_glm::Vec3) -> (f32, f32) {
        let p = (point - self.center).normalize();  // Vector desde el centro de la esfera al punto de intersección
        let theta = p.z.atan2(p.x);  // Ángulo theta
        let phi = p.y.asin();        // Ángulo phi

        let u = 0.5 + theta / (2.0 * std::f32::consts::PI);
        let v = 0.5 - phi / std::f32::consts::PI;

        (u, v)
    }
}

impl RayIntersect for Sphere {
    fn ray_intersect(&self, ray_origin: &nalgebra_glm::Vec3, ray_direction: &nalgebra_glm::Vec3) -> Intersect {
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
                let (u, v) = self.get_uv(&point);

                return Intersect::new(point, normal, distance, self.material.clone(), u, v);
            }
        }
        Intersect::empty()
    }

    fn get_uv(&self, point: &Vector3<f32>) -> (f32, f32) {
        let p = (point - self.center).normalize();
        let theta = p.z.atan2(p.x);  // Ángulo theta
        let phi = p.y.asin();        // Ángulo phi
        let u = 0.5 + theta / (2.0 * std::f32::consts::PI);
        let v = 0.5 - phi / std::f32::consts::PI;
        (u, v)
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
