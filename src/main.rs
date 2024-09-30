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
use std::path::Path;
use crate::texture::Texture;
use crate::ray_intersect::RayIntersect;
use crate::cube::Cube;

use winit::event::{Event, WindowEvent, ElementState, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn main() {
    // Inicializa winit y crea una ventana
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Minecraft Diorama Raytracing")
        .build(&event_loop)
        .unwrap();

    // Inicializamos el framebuffer
    let mut framebuffer = Framebuffer::new(800, 600);
    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

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
    
    // Renderizamos la escena, pasando la cámara y la luz como parámetros
    // Mostrar el framebuffer en una ventana emergente
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { event, .. } => { // Actualizado a la sintaxis correcta
                    if let Some(keycode) = event.virtual_keycode {
                        if event.state == ElementState::Pressed {
                            match keycode {
                                VirtualKeyCode::Up => {
                                    camera.zoom_in(0.2); // Acercamos la cámara
                                }
                                VirtualKeyCode::Down => {
                                    camera.zoom_out(0.2); // Alejamos la cámara
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                // Renderizamos la escena con los nuevos parámetros de la cámara
                raytracer::render(&mut framebuffer, &objects, &camera, &light);
                framebuffer.display();
            }
            _ => (),
        }
    });
}
