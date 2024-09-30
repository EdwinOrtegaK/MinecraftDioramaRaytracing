mod framebuffer;
mod raytracer;
mod ray_intersect;
mod camera;
mod light;
mod color;
mod texture;

use framebuffer::Framebuffer;
use nalgebra::Vector3;
use ray_intersect::{Material, Sphere};
use camera::Camera;
use light::Light;
use color::Color;
use std::f32::consts::PI;
use std::path::Path;
use crate::texture::Texture;

fn main() {
    // Inicializamos el framebuffer
    let mut framebuffer = Framebuffer::new(800, 600);
    framebuffer.set_background_color(0x000000); // Fondo negro
    framebuffer.clear();

    // Definimos las texturas a utilizar
    let head_texture = Texture::load_from_file("assets/agua.jpg");

    // Definimos la cámara
    let mut camera = Camera::new(
        Vector3::new(0.0, 0.0, 5.0),  // Posición de la cámara
        Vector3::new(0.0, 0.0, 0.0),  // Punto que la cámara está mirando (centro de la escena)
        Vector3::new(0.0, 1.0, 0.0),  // Vector "up"
    );
    camera.orbit(PI / 10.0, PI / 20.0);  // Rotamos la cámara

    // Definimos la luz
    let light = Light::new(
        Vector3::new(5.0, 5.0, 5.0),  // Posición de la luz
        Color::new(255, 255, 255),    // Color de la luz (blanco)
        1.0,                          // Intensidad de la luz
    );

    // Definimos los materiales con albedo y reflectividad
    let blue = Material::new(Color::new(0, 0, 255), 32.0, [0.9, 0.1, 0.1, 0.0], 1.0, false, None);  // Azul, mayormente difuso, no transparente
    let white = Material::new(Color::new(255, 255, 255), 32.0, [0.9, 0.1, 0.1, 0.0], 1.0, false, None);  // Blanco, algo reflejado, no transparente
    let black = Material::new(Color::new(0, 0, 0), 32.0, [0.9, 0.1, 0.0, 0.0], 1.0, false, None);  // Negro, no refleja ni es transparente
    let pink = Material::new(Color::new(255, 105, 180), 32.0, [0.8, 0.2, 0.1, 0.0], 1.0, false, None);   // Rosado, algo reflejado, no transparente
    let dark_pink = Material::new(Color::new(199, 21, 133), 32.0, [0.8, 0.2, 0.1, 0.0], 1.0, false, None); // Rosado oscuro, algo reflejado, no transparente

    // Ejemplo de un material transparente (por ejemplo, vidrio)
    let glass = Material::new(Color::new(255, 255, 255), 125.0, [0.0, 0.5, 0.1, 0.8], 1.5, false, None); // Vidrio, 80% transparente, índice de refracción 1.5

    // Ejemplo del uso de textura
    let blue_with_texture = Material::new(Color::new(0, 0, 255), 32.0, [0.9, 0.1, 0.1, 0.0], 1.0, true, Some(head_texture));
    
    // Creamos la esfera azul que será la cabeza de Popplio
    // Añadimos las esferas blancas para los ojos, negras para las pupilas, y azules para las orejas
    let objects = vec![
        // Cabeza azul
        ray_intersect::Sphere {
            center: Vector3::new(0.0, 0.0, -5.0),
            radius: 1.5,
            material: blue_with_texture,
        },
        // Ojo izquierdo (desde la perspectiva del observador)
        ray_intersect::Sphere {
            center: Vector3::new(-0.5, 0.0, -4.2),
            radius: 0.7,
            material: white.clone(),
        },
        // Ojo derecho (desde la perspectiva del observador)
        ray_intersect::Sphere {
            center: Vector3::new(0.5, 0.0, -4.2),
            radius: 0.7,
            material: white.clone(),
        },
        // Pupila izquierda (parte superior)
        ray_intersect::Sphere {
            center: Vector3::new(-0.55, 0.04, -3.0),
            radius: 0.25,
            material: black.clone(),
        },
        // Pupila izquierda (parte inferior)
        ray_intersect::Sphere {
            center: Vector3::new(-0.55, -0.04, -3.0),
            radius: 0.25,
            material: black.clone(),
        },
        // Pupila derecha (parte superior)
        ray_intersect::Sphere {
            center: Vector3::new(0.55, 0.04, -3.0),
            radius: 0.25,
            material: black.clone(),
        },
        // Pupila derecha (parte inferior)
        ray_intersect::Sphere {
            center: Vector3::new(0.55, -0.04, -3.0),
            radius: 0.25,
            material: black.clone(),
        },
        // Pupilas blancas adicionales (izquierda parte superior)
        ray_intersect::Sphere {
            center: Vector3::new(-0.55, 0.1, -2.8), // Posicionadas más cerca
            radius: 0.1,
            material: white.clone(),
        },
        // Pupilas blancas adicionales (izquierda parte inferior)
        ray_intersect::Sphere {
            center: Vector3::new(-0.55, 0.06, -2.8), // Posicionadas más cerca
            radius: 0.1,
            material: white.clone(),
        },
        // Pupilas blancas adicionales (derecha parte superior)
        ray_intersect::Sphere {
            center: Vector3::new(0.55, 0.1, -2.8), // Posicionadas más cerca
            radius: 0.1,
            material: white.clone(),
        },
        // Pupilas blancas adicionales (derecha parte inferior)
        ray_intersect::Sphere {
            center: Vector3::new(0.55, 0.06, -2.8), // Posicionadas más cerca
            radius: 0.1,
            material: white.clone(),
        },
        // Oreja izquierda
        ray_intersect::Sphere {
            center: Vector3::new(-1.8, 0.0, -5.0), // Posicionada a la izquierda de la cabeza
            radius: 0.5,
            material: blue.clone(),
        },
        // Oreja derecha
        ray_intersect::Sphere {
            center: Vector3::new(1.8, 0.0, -5.0), // Posicionada a la derecha de la cabeza
            radius: 0.5,
            material: blue.clone(),
        },
        // Nariz - Círculo más grande (parte inferior)
        ray_intersect::Sphere {
            center: Vector3::new(0.0, -0.5, -3.5), // Bajado y centrado entre los ojos
            radius: 0.35,
            material: white.clone(),
        },
        // Nariz - Círculo mediano (parte media)
        ray_intersect::Sphere {
            center: Vector3::new(0.0, -0.2, -3.3), // Un poco más abajo
            radius: 0.25,
            material: white.clone(),
        },
        // Nariz - Círculo pequeño (parte superior)
        ray_intersect::Sphere {
            center: Vector3::new(0.0, 0.0, -3.1), // Más abajo y más pequeño
            radius: 0.15,
            material: white.clone(),
        },
        // Nariz rosada
        ray_intersect::Sphere {
            center: Vector3::new(0.0, 0.15, -3.05), // Posición más arriba y ligeramente más adelante
            radius: 0.2,
            material: pink.clone(),
        },
        // Boca rosada oscura
        ray_intersect::Sphere {
            center: Vector3::new(0.0, -0.55, -3.4), // Posicionada entre los círculos de la nariz
            radius: 0.25,
            material: dark_pink.clone(),
        },
    ];

    // Renderizamos la escena, pasando la cámara y la luz como parámetros
    raytracer::render(&mut framebuffer, &objects, &camera, &light);

    // Mostrar el framebuffer en una ventana emergente
    framebuffer.display();
}
