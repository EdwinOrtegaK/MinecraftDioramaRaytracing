use nalgebra::Vector3;
use crate::framebuffer::Framebuffer;
use crate::ray_intersect::{Intersect, RayIntersect, Material};

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
            return Intersect::new(distance, self.material);
        }
        Intersect::empty()
    }
}

pub fn render(framebuffer: &mut Framebuffer, objects: &[Sphere]) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;
            let screen_x = screen_x * aspect_ratio;

            let ray_direction = (Vector3::new(screen_x, screen_y, -1.0)).normalize();
            let pixel_color = cast_ray(&Vector3::new(0.0, 0.0, 0.0), &ray_direction, objects);

            framebuffer.set_current_color(pixel_color);
            framebuffer.point(x, y);
        }
    }
}

pub fn cast_ray(ray_origin: &Vector3<f32>, ray_direction: &Vector3<f32>, objects: &[Sphere]) -> u32 {
    let mut intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;

    for object in objects {
        let tmp = object.ray_intersect(ray_origin, ray_direction);
        if tmp.is_intersecting && tmp.distance < zbuffer {
            zbuffer = tmp.distance;
            intersect = tmp;
        }
    }

    if !intersect.is_intersecting {
        return 0x040C24;
    }

    intersect.material.diffuse
}
