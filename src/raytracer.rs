use nalgebra::Vector3;
use crate::framebuffer::Framebuffer;
use crate::ray_intersect::{Intersect, RayIntersect, Material};
use crate::camera::Camera;
use crate::light::Light;

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
            let point = ray_origin + ray_direction * distance;
            let normal = (point - self.center).normalize();
            return Intersect::new(point, normal, distance, self.material);
        }
        Intersect::empty()
    }
}

pub fn render(framebuffer: &mut Framebuffer, objects: &[Sphere], camera: &Camera, light: &Light) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;
            let screen_x = screen_x * aspect_ratio;

            let ray_direction = camera.basis_change(&Vector3::new(screen_x, screen_y, -1.0).normalize());

            let pixel_color = cast_ray(&camera.eye, &ray_direction, objects, light);

            framebuffer.set_current_color(pixel_color);
            framebuffer.point(x, y);
        }
    }
}

pub fn cast_ray(ray_origin: &Vector3<f32>, ray_direction: &Vector3<f32>, objects: &[Sphere], light: &Light) -> u32 {
    let mut closest_intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;

    for object in objects {
        let intersect = object.ray_intersect(ray_origin, ray_direction);
        if intersect.is_intersecting && intersect.distance < zbuffer {
            zbuffer = intersect.distance;
            closest_intersect = intersect;
        }
    }

    if !closest_intersect.is_intersecting {
        return 0x040C24;  // Color de fondo (negro)
    }

    // Calcular la dirección hacia la luz
    let light_dir = (light.position - closest_intersect.point).normalize();

    // Calcular la iluminación difusa (ley del coseno de Lambert)
    let diffuse_intensity = light_dir.dot(&closest_intersect.normal).max(0.0);
    let final_intensity = light.intensity * diffuse_intensity;

    // Modificar el color del material según la intensidad de la luz y el color de la luz
    let material_color = closest_intersect.material.diffuse;
    let r = ((material_color >> 16) & 0xFF) as f32 * (light.color.r as f32 / 255.0) * final_intensity;
    let g = ((material_color >> 8) & 0xFF) as f32 * (light.color.g as f32 / 255.0) * final_intensity;
    let b = (material_color & 0xFF) as f32 * (light.color.b as f32 / 255.0) * final_intensity;

    ((r.min(255.0) as u32) << 16) | ((g.min(255.0) as u32) << 8) | (b.min(255.0) as u32)
}
