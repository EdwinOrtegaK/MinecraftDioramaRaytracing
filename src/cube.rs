use nalgebra_glm::Vec3;
use crate::ray_intersect::{Intersect, RayIntersect, Material};
use image::RgbaImage;
use crate::color::Color;
use crate::light::Light; 
use std::any::Any;

pub struct Cube {
    pub center: Vec3,
    pub size: f32,
    pub materials: [Material; 6], 
}

impl Cube {
    pub fn new(center: Vec3, size: f32, materials: [Material; 6]) -> Self {
        Cube { center, size, materials }
    }

    fn get_uv(&self, punto_encuentro: &Vec3, normal: &Vec3) -> (f32, f32) {
        let mitad = self.size / 2.0;
        let min = self.center - Vec3::new(mitad, mitad, mitad);
        let max = self.center + Vec3::new(mitad, mitad, mitad);
    
        let (mut u, mut v) = if normal.x.abs() > 0.9 {
            // Caras laterales (+X, -X)
            (
                (punto_encuentro.z - min.z) / (max.z - min.z),
                (punto_encuentro.y - min.y) / (max.y - min.y)
            )
        } else if normal.y.abs() > 0.9 {
            // Caras superiores/inferiores (+Y, -Y)
            (
                (punto_encuentro.x - min.x) / (max.x - min.x),
                (punto_encuentro.z - min.z) / (max.z - min.z)
            )
        } else {
            // Caras frontales/traseras (+Z, -Z)
            (
                (punto_encuentro.x - min.x) / (max.x - min.x),
                (punto_encuentro.y - min.y) / (max.y - min.y)
            )
        };
    
        // Escalar las coordenadas UV para asegurarnos de que caigan dentro del rango esperado
        u *= 5.0; // Ajustar este valor si es necesario
        v *= 5.0; // Ajustar este valor si es necesario
    
        (u.clamp(0.0, 1.0), v.clamp(0.0, 1.0))
    }
      

    fn get_diffuse_color(&self, face_index: usize, u: f32, v: f32) -> Color {
        if let Some(textura) = &self.materials[face_index].texture {
            let tex_x = (u * textura.width() as f32).round() as usize % textura.width();
            let tex_y = (v * textura.height() as f32).round() as usize % textura.height();
            let pixel = textura.get_pixel(tex_x, tex_y);
            Color::new(pixel.r(), pixel.g(), pixel.b()) // Accede a los componentes del color
        } else {
            self.materials[face_index].diffuse.clone()
        }
    }    
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ray_origin: &nalgebra_glm::Vec3, ray_direction: &nalgebra_glm::Vec3) -> Intersect {
        let mitad = self.size / 2.0;
        let min = self.center - Vec3::new(mitad, mitad, mitad);
        let max = self.center + Vec3::new(mitad, mitad, mitad);

        let inv_dir = Vec3::new(1.0 / ray_direction.x, 1.0 / ray_direction.y, 1.0 / ray_direction.z);
        let t_min = (min - ray_origin).component_mul(&inv_dir);
        let t_max = (max - ray_origin).component_mul(&inv_dir);

        let t1 = t_min.x.min(t_max.x).max(t_min.y.min(t_max.y)).max(t_min.z.min(t_max.z));
        let t2 = t_min.x.max(t_max.x).min(t_min.y.max(t_max.y)).min(t_min.z.max(t_max.z));

        if t1 > t2 || t2 < 0.0 {
            return Intersect::empty();
        }

        let t_hit = if t1 < 0.0 { t2 } else { t1 };
        let punto_encuentro = ray_origin + ray_direction * t_hit;

        let mut normal = Vec3::new(0.0, 0.0, 0.0);
        let mut face_index = 0;

        // Detectar cuál cara es y asignar la normal adecuada
        if (punto_encuentro.x - min.x).abs() < 0.001 {
            normal.x = -1.0;
            face_index = 0; // -X
        } else if (punto_encuentro.x - max.x).abs() < 0.001 {
            normal.x = 1.0;
            face_index = 1; // +X
        } else if (punto_encuentro.y - min.y).abs() < 0.001 {
            normal.y = -1.0;
            face_index = 2; // -Y   
        } else if (punto_encuentro.y - max.y).abs() < 0.001 {
            normal.y = 1.0;
            face_index = 3; // +Y
        } else if (punto_encuentro.z - min.z).abs() < 0.001 {
            normal.z = -1.0;
            face_index = 4; // -Z
        } else if (punto_encuentro.z - max.z).abs() < 0.001 {
            normal.z = 1.0;
            face_index = 5; // +Z
        }

        // Aquí verificamos si la normal es correcta o no
        if normal.dot(ray_direction) > 0.0 {
            normal = -normal; // Invertimos la normal si está mal orientada
        }

        let (u, v) = self.get_uv(&punto_encuentro, &normal);
        println!("UV coordinates for face {}: u = {}, v = {}", face_index, u, v);

        Intersect::new(
            punto_encuentro,
            normal,
            t_hit,
            self.materials[face_index].clone(),
            u,
            v
        )
    }

    fn get_uv(&self, point: &nalgebra_glm::Vec3) -> (f32, f32) {
        let p = (point - self.center).normalize();
        let theta = p.z.atan2(p.x);
        let phi = p.y.asin();
        let u = 0.5 + theta / (2.0 * std::f32::consts::PI);
        let v = 0.5 - phi / std::f32::consts::PI;
        (u, v)
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
