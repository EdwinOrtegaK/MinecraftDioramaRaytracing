use nalgebra::Vector3;
use crate::framebuffer::Framebuffer;
use crate::ray_intersect::{Intersect, RayIntersect, Material};
use crate::camera::Camera;
use crate::light::Light;
use crate::color::Color;

pub fn render(framebuffer: &mut Framebuffer, objects: &[Box<dyn RayIntersect>], camera: &Camera, lights: &[Light]) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;
            let screen_x = screen_x * aspect_ratio;

            let ray_direction = camera.base_change(&Vector3::new(screen_x, screen_y, -1.0).normalize());

            // Inicializamos el color del píxel como negro (0, 0, 0)
            let mut pixel_color = Color::new(0, 0, 0);

            // Sumamos el color generado por cada luz
            for light in lights {
                let light_contrib = cast_ray(&camera.eye, &ray_direction, objects, light, 0);  // Calculamos la contribución de la luz
            
                // Actualizamos cada componente del color manualmente y nos aseguramos de que no exceda 255
                pixel_color.r = (pixel_color.r + light_contrib.r).min(255);
                pixel_color.g = (pixel_color.g + light_contrib.g).min(255);
                pixel_color.b = (pixel_color.b + light_contrib.b).min(255);
            }

            framebuffer.set_current_color(pixel_color.to_u32());
            framebuffer.point(x, y);
        }
    }
}


fn cast_shadow(
    intersect: &Intersect,
    light: &Light,
    objects: &[Box<dyn RayIntersect>],
) -> f32 {
    let light_dir = (light.position - intersect.point).normalize();
    
    let shadow_ray_origin = intersect.point + intersect.normal * 1e-2;
    let mut shadow_intensity = 0.0;

    for object in objects {
        let shadow_intersect = object.ray_intersect(&shadow_ray_origin, &light_dir);
        if shadow_intersect.is_intersecting {
            let distance_to_light = (light.position - intersect.point).magnitude();
            let shadow_distance = (shadow_intersect.point - shadow_ray_origin).magnitude();
            
            shadow_intensity = (1.0 - (shadow_distance / distance_to_light)).max(0.0);
            break;
        }
    }

    shadow_intensity
}

fn refract(incident: &Vector3<f32>, normal: &Vector3<f32>, eta_t: f32) -> Vector3<f32> {
    let cosi = -incident.dot(normal).max(-1.0).min(1.0);

    let (n_cosi, eta, n_normal);

    if cosi < 0.0 {
        // El rayo está entrando en el objeto
        n_cosi = -cosi;
        eta = 1.0 / eta_t;
        n_normal = -normal;
    } else {
        // El rayo está saliendo del objeto
        n_cosi = cosi;
        eta = eta_t;
        n_normal = *normal;
    }

    let k = 1.0 - eta * eta * (1.0 - n_cosi * n_cosi);

    if k < 0.0 {
        // Reflexión total interna
        reflect(incident, &n_normal)
    } else {
        eta * incident + (eta * n_cosi - k.sqrt()) * n_normal
    }
}

pub fn cast_ray(
    ray_origin: &Vector3<f32>,
    ray_direction: &Vector3<f32>,
    objects: &[Box<dyn RayIntersect>],
    light: &Light,
    depth: u32,
) -> Color {
    if depth > 3 {
        return Color::new(0, 0, 0);  // Color de fondo si alcanzamos la profundidad máxima
    }

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
        return Color::new(4, 12, 36);  // Color de fondo
    }

    let diffuse_color = closest_intersect.material.get_diffuse_color(closest_intersect.u, closest_intersect.v);
    let light_dir = (light.position - closest_intersect.point).normalize();
    let view_dir = (ray_origin - closest_intersect.point).normalize();
    let reflect_dir = reflect(&-light_dir, &closest_intersect.normal);

    let shadow_intensity = cast_shadow(&closest_intersect, light, objects);
    let light_intensity = light.intensity * (1.0 - shadow_intensity);

    let diffuse_intensity = light_dir.dot(&closest_intersect.normal).max(0.0).min(1.0);
    let diffuse = diffuse_color.scale(closest_intersect.material.albedo[0] * diffuse_intensity * light_intensity);

    let specular_intensity = view_dir.dot(&reflect_dir).max(0.0).powf(closest_intersect.material.specular);
    let specular = light.color.scale(closest_intersect.material.albedo[1] * specular_intensity * light_intensity);

    let mut reflect_color = Color::new(0, 0, 0);
    let reflectivity = closest_intersect.material.albedo[2];
    if reflectivity > 0.0 {
        let reflect_dir = reflect(&-ray_direction, &closest_intersect.normal).normalize();
        let reflect_origin = closest_intersect.point + closest_intersect.normal * 1e-3;
        reflect_color = cast_ray(&reflect_origin, &reflect_dir, objects, light, depth + 1);
    }

    let mut refract_color = Color::new(0, 0, 0);
    let transparency = closest_intersect.material.albedo[3];
    if transparency > 0.0 {
        let refract_dir = refract(&ray_direction, &closest_intersect.normal, closest_intersect.material.refractive_index).normalize();
        let refract_origin = closest_intersect.point - closest_intersect.normal * 1e-3;
        refract_color = cast_ray(&refract_origin, &refract_dir, objects, light, depth + 1);
    }

    (diffuse + specular)
        .scale(1.0 - reflectivity - transparency)
        + reflect_color.scale(reflectivity)
        + refract_color.scale(transparency)
}

fn reflect(incident: &Vector3<f32>, normal: &Vector3<f32>) -> Vector3<f32> {
    incident - 2.0 * incident.dot(normal) * normal
}
