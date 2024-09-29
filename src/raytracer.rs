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
            if distance > 0.0 {
                let point = ray_origin + ray_direction * distance;
                let normal = (point - self.center).normalize();
                return Intersect::new(point, normal, distance, self.material);
            }
        }
        Intersect::empty()
    }
}

pub fn reflect(incident: &Vector3<f32>, normal: &Vector3<f32>) -> Vector3<f32> {
    incident - 2.0 * incident.dot(normal) * normal
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

    // Buscamos la intersección más cercana con los objetos
    for object in objects {
        let intersect = object.ray_intersect(ray_origin, ray_direction);
        if intersect.is_intersecting && intersect.distance < zbuffer {
            zbuffer = intersect.distance;
            closest_intersect = intersect;
        }
    }

    // Si no hay intersección, devolvemos el color de fondo
    if !closest_intersect.is_intersecting {
        return 0x040C24;  // Color de fondo (negro)
    }

    // Calcular la dirección hacia la luz
    let light_dir = (light.position - closest_intersect.point).normalize();
    let view_dir = (ray_origin - closest_intersect.point).normalize();

    // Calcular la reflexión de la luz sobre la superficie
    let reflect_dir = reflect(&-light_dir, &closest_intersect.normal);

    // Iluminación difusa (Ley del coseno de Lambert)
    let diffuse_intensity = light_dir.dot(&closest_intersect.normal).max(0.0);
    let diffuse_color = closest_intersect.material.diffuse;
    let diffuse_r = ((diffuse_color.r as f32) * diffuse_intensity * light.intensity).min(255.0);
    let diffuse_g = ((diffuse_color.g as f32) * diffuse_intensity * light.intensity).min(255.0);
    let diffuse_b = ((diffuse_color.b as f32) * diffuse_intensity * light.intensity).min(255.0);

    // Iluminación especular
    let specular_intensity = view_dir.dot(&reflect_dir).max(0.0).powf(closest_intersect.material.specular);
    let specular_r = ((light.color.r as f32) * specular_intensity * light.intensity).min(255.0);
    let specular_g = ((light.color.g as f32) * specular_intensity * light.intensity).min(255.0);
    let specular_b = ((light.color.b as f32) * specular_intensity * light.intensity).min(255.0);

    // Combinamos los componentes difusos y especulares
    let r = (diffuse_r + specular_r).min(255.0) as u32;
    let g = (diffuse_g + specular_g).min(255.0) as u32;
    let b = (diffuse_b + specular_b).min(255.0) as u32;

    // Devolvemos el color calculado como un valor RGB en formato u32
    (r << 16) | (g << 8) | b
}
