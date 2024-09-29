use nalgebra::Vector3;
use crate::framebuffer::Framebuffer;
use crate::ray_intersect::{Intersect, RayIntersect, Material};
use crate::camera::Camera;
use crate::light::Light;
use crate::color::Color;

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

            let pixel_color = cast_ray(&camera.eye, &ray_direction, objects, light, 0);  // Pasamos 0 como profundidad inicial

            framebuffer.set_current_color(pixel_color.to_u32());
            framebuffer.point(x, y);
        }
    }
}

fn cast_shadow(
    intersect: &Intersect,
    light: &Light,
    objects: &[Sphere],
) -> f32 {
    let light_dir = (light.position - intersect.point).normalize();
    
    // Incrementamos el desplazamiento para evitar el acné de sombra
    let shadow_ray_origin = intersect.point + intersect.normal * 1e-2;
    let mut shadow_intensity = 0.0;

    for object in objects {
        let shadow_intersect = object.ray_intersect(&shadow_ray_origin, &light_dir);
        if shadow_intersect.is_intersecting {
            let distance_to_light = (light.position - intersect.point).magnitude();
            let shadow_distance = (shadow_intersect.point - shadow_ray_origin).magnitude();
            
            // Ajuste para suavizar sombras en función de la distancia
            shadow_intensity = (1.0 - (shadow_distance / distance_to_light)).max(0.0);
            break;
        }
    }

    shadow_intensity
}

pub fn cast_ray(ray_origin: &Vector3<f32>, ray_direction: &Vector3<f32>, objects: &[Sphere], light: &Light, depth: u32) -> Color {
    if depth > 3 {
        return Color::new(0, 0, 0);  // Color de fondo si alcanzamos la profundidad máxima de reflexión
    }

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
        return Color::new(4, 12, 36);  // Color de fondo
    }

    // Calcular la dirección hacia la luz
    let light_dir = (light.position - closest_intersect.point).normalize();
    let view_dir = (ray_origin - closest_intersect.point).normalize();
    let reflect_dir = reflect(&-light_dir, &closest_intersect.normal);

    // Calcular sombras
    let shadow_intensity = cast_shadow(&closest_intersect, light, objects);
    let light_intensity = light.intensity * (1.0 - shadow_intensity);

    // Iluminación difusa
    let diffuse_intensity = light_dir.dot(&closest_intersect.normal).max(0.0).min(1.0);
    let diffuse = closest_intersect.material.diffuse.scale(closest_intersect.material.albedo[0] * diffuse_intensity * light_intensity);

    // Iluminación especular
    let specular_intensity = view_dir.dot(&reflect_dir).max(0.0).powf(closest_intersect.material.specular);
    let specular = light.color.scale(closest_intersect.material.albedo[1] * specular_intensity * light_intensity);

    // Reflexiones
    let mut reflect_color = Color::new(0, 0, 0);
    let reflectivity = closest_intersect.material.albedo[2];
    if reflectivity > 0.0 {
        let reflect_dir = reflect(&-ray_direction, &closest_intersect.normal).normalize();
        let reflect_origin = closest_intersect.point + closest_intersect.normal * 1e-3;
        reflect_color = cast_ray(&reflect_origin, &reflect_dir, objects, light, depth + 1);
    }

    // Combinamos los componentes difusos, especulares y reflejados
    (diffuse + specular).scale(1.0 - reflectivity) + reflect_color.scale(reflectivity)
}

fn reflect(incident: &Vector3<f32>, normal: &Vector3<f32>) -> Vector3<f32> {
    incident - 2.0 * incident.dot(normal) * normal
}
