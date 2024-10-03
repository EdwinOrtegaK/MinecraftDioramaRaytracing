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
    framebuffer.set_background_color(0x404040);
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
    let tierra_texture = Texture::load_from_file("assets/tierra.png");
    let tierra_grama_texture = Texture::load_from_file("assets/tierra2.png");
    let grama_texture = Texture::load_from_file("assets/grama.png");
    let madera_texture = Texture::load_from_file("assets/madera.jpg");
    let hoja_texture = Texture::load_from_file("assets/hoja_arbol.jpg");
    let piedra_texture = Texture::load_from_file("assets/piedra.webp");
    let lava_texture = Texture::load_from_file("assets/lava.jpg");

    //let textura_solida = Color::new(255, 0, 0);
    //let material_prueba = Material::new(textura_solida, 32.0, [1.0, 0.1, 0.0, 0.0], 1.0, false, None);

    // Definimos la cámara
    let mut camera = Camera::new(
        Vector3::new(0.0, 0.0, -10.0),  // Posición de la cámara
        Vector3::new(0.0, 0.0, 0.0),  // Punto que la cámara está mirando (centro de la escena)
        Vector3::new(0.0, 1.0, 0.0),  // Vector "up"
    );

    // Definimos la luz
    let lights = vec![
        Light::new(Vector3::new(100.0, 100.0, -100.0), Color::new(255, 255, 255), 2.0, 5.0), 
        Light::new(Vector3::new(-100.0, -100.0, 100.0), Color::new(255, 255, 255), 2.0, 5.0),
    ];

    // Definimos los materiales 
    let tierra_grama = Material::new(Color::new(255, 255, 255), 32.0, [0.9, 0.1, 0.0, 0.0], 1.0, true, Some(tierra_grama_texture.clone()));
    let tierra = Material::new(Color::new(255, 255, 255), 32.0, [1.0, 0.1, 0.0, 0.0], 1.0, true, Some(tierra_texture.clone()));
    let grama = Material::new(Color::new(255, 255, 255), 32.0, [1.0, 0.1, 0.0, 0.0], 1.0, true, Some(grama_texture.clone()));
    let agua = Material::new(Color::new(255, 255, 255), 32.0, [1.0, 0.1, 0.0, 0.0], 1.0, true, Some(agua_texture.clone()));
    let madera = Material::new(Color::new(255, 255, 255), 32.0, [1.0, 0.1, 0.0, 0.0], 1.0, true, Some(madera_texture.clone()));
    let piedra = Material::new(Color::new(255, 255, 255), 32.0, [1.0, 0.1, 0.0, 0.0], 1.0, true, Some(piedra_texture.clone()));
    let hoja = Material::new(Color::new(255, 255, 255), 32.0, [1.0, 0.1, 0.0, 0.0], 1.0, true, Some(hoja_texture.clone()));
    let lava = Material::new(Color::new(255, 255, 255), 32.0, [1.0, 0.1, 0.0, 0.0], 1.0, true, Some(lava_texture.clone()));

    // Creamos un cubo en la escena
    let mut objects: Vec<Box<dyn RayIntersect>> = vec![ 
        Box::new(Cube {
            center: Vector3::new(0.0, 0.0, -5.0),  // Posición del cubo
            size: 1.0,                           // Tamaño del cubo
            materials: [
                tierra_grama.clone(),  // Frente
                agua.clone(),  // Atrás
                tierra_grama.clone(),  // Izquierda
                lava.clone(),  // Derecha
                grama.clone(),         // Arriba
                tierra.clone()         // Abajo
            ],
        }),
    ];
    
    /*
    let mut objects: Vec<Box<dyn RayIntersect>> = vec![ 
        Box::new(Cube {
            center: Vector3::new(0.0, 0.0, -5.0),  // Posición del cubo
            size: 1.0,                           // Tamaño del cubo
            materials: [
                material_prueba.clone(),  // Frente
                material_prueba.clone(),  // Atrás
                material_prueba.clone(),  // Izquierda
                material_prueba.clone(),  // Derecha
                material_prueba.clone(),  // Arriba
                material_prueba.clone()   // Abajo
            ],
        }),
    ];
    */

    // Ejemplo de un material transparente (por ejemplo, vidrio)
    let glass = Material::new(Color::new(255, 255, 255), 125.0, [0.0, 0.5, 0.1, 0.8], 1.5, false, None); // Vidrio, 80% transparente, índice de refracción 1.5
    
    let mut needs_render = true;
    
    // Bucle principal para manejar la entrada del teclado y actualizar la cámara
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Renderizamos la escena con los nuevos parámetros de la cámara
        raytracer::render(&mut framebuffer, &objects, &camera, &lights);

        // Muestra el framebuffer en la ventana
        window
            .update_with_buffer(framebuffer.get_buffer(), framebuffer.width, framebuffer.height)
            .unwrap();

        // Procesamos la entrada del teclado para mover la cámara
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

        // Procesamos la entrada del teclado para rotar la cámara
        if window.is_key_down(Key::Up) {
            camera.orbit(0.0, -0.05);    // Rotar hacia arriba
        }
        if window.is_key_down(Key::Down) {
            camera.orbit(0.0, 0.05);     // Rotar hacia abajo
        }
        if window.is_key_down(Key::Left) {
            camera.orbit(-0.05, 0.0);    // Rotar hacia la izquierda
        }
        if window.is_key_down(Key::Right) {
            camera.orbit(0.05, 0.0);     // Rotar hacia la derecha
        }

        // Animar cubos u otros objetos específicos usando downcasting
        for object in objects.iter_mut() {
            // Usamos downcasting dinámico para verificar si el objeto es un Cube
            if let Some(cube) = object.as_any_mut().downcast_mut::<Cube>() {
                // Aquí puedes animar o modificar el cubo
                // Por ejemplo, podemos moverlo un poco en el eje Y para hacer que flote
                // cube.center.y += 0.01; Movimiento hacia arriba
            
                // O puedes aplicar alguna lógica de animación más compleja
                // cube.size *= 1.01;  // Incrementar el tamaño del cubo ligeramente
            }
        }

        // Añadimos un pequeño delay para que no consuma tanto CPU
        std::thread::sleep(Duration::from_millis(16));
    }
}