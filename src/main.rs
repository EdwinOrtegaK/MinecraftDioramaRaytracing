mod framebuffer;
mod raytracer;
mod ray_intersect;
mod camera;
mod light;
mod color;
mod texture;
mod cube;

use framebuffer::Framebuffer;
use nalgebra::Vector3;
use ray_intersect::Material;
use camera::Camera;
use light::Light;
use color::Color;
use std::f32::consts::PI;
use std::time::Duration;
use std::io::{self, Write};
use minifb::{Key, Window, WindowOptions};
use std::path::Path;
use crate::texture::Texture;
use crate::ray_intersect::RayIntersect;
use crate::cube::Cube;


fn main() {
    // Inicializamos el framebuffer
    let mut framebuffer = Framebuffer::new(800, 600);
    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    // Inicializamos la ventana con minifb
    let mut window = Window::new(
        "Minecraft Diorama Raytracing",
        800,
        600,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Definimos las texturas a utilizar
    let agua_texture = Texture::load_from_file("assets/agua.jpg");
    let tierra_texture = Texture::load_from_file("assets/tierra2.png");

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

    // Definimos los materiales para cada cara del cubo
    let cube_materials = [
        Material::new(Color::new(255, 255, 255), 32.0, [0.9, 0.1, 0.1, 0.0], 1.0, true, Some(tierra_texture.clone())),
        Material::new(Color::new(255, 255, 255), 32.0, [0.9, 0.1, 0.1, 0.0], 1.0, true, Some(tierra_texture.clone())),
        Material::new(Color::new(255, 255, 255), 32.0, [0.9, 0.1, 0.1, 0.0], 1.0, true, Some(tierra_texture.clone())),
        Material::new(Color::new(255, 255, 255), 32.0, [0.9, 0.1, 0.1, 0.0], 1.0, true, Some(tierra_texture.clone())),
        Material::new(Color::new(255, 255, 255), 32.0, [0.9, 0.1, 0.1, 0.0], 1.0, true, Some(tierra_texture.clone())),
        Material::new(Color::new(255, 255, 255), 32.0, [0.9, 0.1, 0.1, 0.0], 1.0, true, Some(tierra_texture.clone())),
    ];

    // Creamos un cubo en la escena
    let cube = Cube::new(Vector3::new(0.0, 0.0, -5.0), 2.0, cube_materials);

    // Ejemplo de un material transparente (por ejemplo, vidrio)
    let glass = Material::new(Color::new(255, 255, 255), 125.0, [0.0, 0.5, 0.1, 0.8], 1.5, false, None); // Vidrio, 80% transparente, índice de refracción 1.5
    
    // Reemplazamos el vector de objetos para contener únicamente el cubo
    let objects: Vec<Box<dyn RayIntersect>> = vec![Box::new(cube)];

    let mut needs_render = true;
    
    // Bucle principal para manejar la entrada del teclado y actualizar la cámara
    while window.is_open() && !window.is_key_down(Key::Q) {
        // Renderizamos la escena con los nuevos parámetros de la cámara
        raytracer::render(&mut framebuffer, &objects, &camera, &light);

        // Muestra el framebuffer en la ventana
        window
            .update_with_buffer(framebuffer.get_buffer(), framebuffer.width, framebuffer.height)
            .unwrap();

        // Procesamos la entrada del teclado
        if window.is_key_down(Key::W) {
            camera.mover_enfrente(0.2);  // Mover cámara hacia adelante
        }
        if window.is_key_down(Key::S) {
            camera.mover_atras(0.2);     // Mover cámara hacia atrás
        }
        if window.is_key_down(Key::A) {
            camera.mover_izq(0.2);       // Mover cámara hacia la izquierda
        }
        if window.is_key_down(Key::D) {
            camera.mover_der(0.2);       // Mover cámara hacia la derecha
        }
        if window.is_key_down(Key::I) {
            camera.mover_enfrente(0.2);  // Acercar la cámara
        }
        if window.is_key_down(Key::K) {
            camera.mover_atras(0.2);     // Alejar la cámara
        }

        // Solo renderizamos cuando sea necesario (cuando la cámara se mueva)
        if needs_render {
            // Renderizamos la escena con los nuevos parámetros de la cámara
            raytracer::render(&mut framebuffer, &objects, &camera, &light);

            // Muestra el framebuffer en la ventana
            window
                .update_with_buffer(framebuffer.get_buffer(), framebuffer.width, framebuffer.height)
                .unwrap();

            needs_render = false; // Reseteamos la bandera después de renderizar
        }

        // Añadimos un pequeño delay para que no consuma tanto CPU
        std::thread::sleep(Duration::from_millis(16));
    }
}
