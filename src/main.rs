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
use crate::raytracer::render;
use crate::texture::Texture;
use crate::ray_intersect::RayIntersect;
use crate::cube::Cube;


fn main() {
    let width = 800;
    let height = 600;

    // Initialize framebuffers
    let mut framebuffer_high = Framebuffer::new(width, height);
    let mut framebuffer_low = Framebuffer::new(width / 3, height / 3);

    // Inicializamos la ventana con minifb
    let mut window = Window::new(
        "Minecraft Diorama Raytracing",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Definimos las texturas a utilizar
    let agua_texture = Texture::load_from_file("assets/agua.jpg");
    let tierra_texture = Texture::load_from_file("assets/tierra.jpeg");
    let tierra_grama_texture = Texture::load_from_file("assets/tierra2.png");
    let grama_texture = Texture::load_from_file("assets/grama.png");
    let madera_texture = Texture::load_from_file("assets/madera.jpg");
    let hoja_texture = Texture::load_from_file("assets/hoja_arbol.jpg");
    let piedra_texture = Texture::load_from_file("assets/piedra.png");
    let arena_texture = Texture::load_from_file("assets/arena.png");

    //let textura_solida = Color::new(255, 0, 0);
    //let material_prueba = Material::new(textura_solida, 32.0, [1.0, 0.1, 0.0, 0.0], 1.0, false, None);

    // Definimos la cámara
    let mut camera = Camera::new(
        Vector3::new(0.0, 5.0, -10.0),  // Posición de la cámara
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
    let arena = Material::new(Color::new(255, 255, 255), 32.0, [1.0, 0.1, 0.0, 0.0], 1.0, true, Some(arena_texture.clone()));

    // Creamos los cubos en la escena
    let mut objects: Vec<Box<dyn RayIntersect>> = Vec::new();

    // Base de 8x8 cubos
    let grid_size = 8;
    let cube_size = 1.0;
    
    // Recorrer en ambos ejes X y Z para crear una cuadrícula
    for z in 0..grid_size {
        for x in 0..grid_size {
            let x_pos = x as f32 * cube_size - (grid_size as f32 / 2.0) * cube_size;
            let z_pos = z as f32 * cube_size - (grid_size as f32 / 2.0) * cube_size;
            
            objects.push(Box::new(Cube {
                center: Vector3::new(x_pos, 0.0, z_pos),  // Posición del cubo
                size: cube_size,                         // Tamaño del cubo
                materials: [
                    tierra.clone(),  // Derecha
                    tierra.clone(),  // Izquierda
                    tierra.clone(),  // Abajo
                    tierra.clone(),  // Arriba
                    tierra.clone(),  // Frente
                    tierra.clone(),  // Atrás
                ],
            }));
        }
    }
    objects.push(Box::new(Cube {
        center: Vector3::new(3.0, 1.0, 3.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(3.0, 1.0, 2.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(3.0, 1.0, 1.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(2.0, 1.0, 1.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(2.0, 1.0, 0.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(1.0, 1.0, 0.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(1.0, 1.0, -1.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(0.0, 1.0, -1.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(0.0, 1.0, -2.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-1.0, 1.0, -2.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-1.0, 1.0, -3.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-2.0, 1.0, -3.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-2.0, 1.0, -4.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-3.0, 1.0, -4.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-4.0, 1.0, -4.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-4.0, 1.0, -3.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-4.0, 1.0, -2.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-4.0, 1.0, -1.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-3.0, 1.0, -1.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-3.0, 1.0, 0.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-2.0, 1.0, 0.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-2.0, 1.0, 1.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-1.0, 1.0, 1.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-1.0, 1.0, 2.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(0.0, 1.0, 2.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(0.0, 1.0, 3.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(1.0, 1.0, 3.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(2.0, 1.0, 3.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(2.0, 1.0, 2.0),  
        size: 1.0,                          
        materials: [
            agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(1.0, 1.0, 2.0),  
        size: 1.0,                          
        materials: [
            agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(1.0, 1.0, 1.0),  
        size: 1.0,                          
        materials: [
            agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(0.0, 1.0, 1.0),  
        size: 1.0,                          
        materials: [
            agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(0.0, 1.0, 0.0),  
        size: 1.0,                          
        materials: [
            agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-1.0, 1.0, 0.0),  
        size: 1.0,                          
        materials: [
            agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-1.0, 1.0, -1.0),  
        size: 1.0,                          
        materials: [
            agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-2.0, 1.0, -1.0),  
        size: 1.0,                          
        materials: [
            agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-2.0, 1.0, -2.0),  
        size: 1.0,                          
        materials: [
            agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-3.0, 1.0, -2.0),  
        size: 1.0,                          
        materials: [
            agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-3.0, 1.0, -3.0),  
        size: 1.0,                          
        materials: [
            agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(3.0, 1.0, 0.0),  
        size: 1.0,                          
        materials: [
            tierra.clone(), tierra_grama.clone(), tierra.clone(), grama.clone(), tierra.clone(), tierra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(3.0, 1.0, -1.0),  
        size: 1.0,                          
        materials: [
            tierra.clone(), tierra_grama.clone(), tierra.clone(), grama.clone(), tierra.clone(), tierra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(3.0, 1.0, -2.0),  
        size: 1.0,                          
        materials: [
            tierra.clone(), tierra.clone(), tierra.clone(), grama.clone(), tierra.clone(), tierra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(3.0, 1.0, -3.0),  
        size: 1.0,                          
        materials: [
            tierra.clone(), tierra.clone(), tierra.clone(), grama.clone(), tierra.clone(), tierra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(3.0, 1.0, -4.0),  
        size: 1.0,                          
        materials: [
            tierra.clone(), tierra.clone(), tierra.clone(), grama.clone(), tierra.clone(), tierra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(2.0, 1.0, -4.0),  
        size: 1.0,                          
        materials: [
            tierra.clone(), tierra.clone(), tierra.clone(), grama.clone(), tierra.clone(), tierra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(1.0, 1.0, -4.0),  
        size: 1.0,                          
        materials: [
            tierra.clone(), tierra.clone(), tierra.clone(), grama.clone(), tierra.clone(), tierra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(0.0, 1.0, -4.0),  
        size: 1.0,                          
        materials: [
            tierra.clone(), tierra.clone(), tierra.clone(), grama.clone(), tierra_grama.clone(), tierra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-1.0, 1.0, -4.0),  
        size: 1.0,                          
        materials: [
            tierra.clone(), tierra.clone(), tierra.clone(), grama.clone(), tierra_grama.clone(), tierra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(2.0, 1.0, -1.0),  
        size: 1.0,                          
        materials: [
            tierra.clone(), tierra.clone(), tierra.clone(), grama.clone(), tierra.clone(), tierra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(2.0, 1.0, -2.0),  
        size: 1.0,                          
        materials: [
            tierra.clone(), tierra.clone(), tierra.clone(), grama.clone(), tierra.clone(), tierra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(1.0, 1.0, -2.0),  
        size: 1.0,                          
        materials: [
            tierra.clone(), tierra.clone(), tierra.clone(), grama.clone(), tierra.clone(), tierra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(1.0, 1.0, -3.0),  
        size: 1.0,                          
        materials: [
            tierra.clone(), tierra.clone(), tierra.clone(), grama.clone(), tierra.clone(), tierra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(0.0, 1.0, -3.0),  
        size: 1.0,                          
        materials: [
            tierra.clone(), tierra.clone(), tierra.clone(), grama.clone(), tierra.clone(), tierra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(3.0, 2.0, -2.0),  
        size: 1.0,                          
        materials: [
            tierra_grama.clone(), tierra_grama.clone(), tierra.clone(), grama.clone(), tierra_grama.clone(), tierra_grama.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(3.0, 2.0, -3.0),  
        size: 1.0,                          
        materials: [
            tierra_grama.clone(), tierra_grama.clone(), tierra.clone(), grama.clone(), tierra_grama.clone(), tierra_grama.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(3.0, 2.0, -4.0),  
        size: 1.0,                          
        materials: [
            tierra_grama.clone(), tierra_grama.clone(), tierra.clone(), grama.clone(), tierra_grama.clone(), tierra_grama.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(2.0, 2.0, -3.0),  
        size: 1.0,                          
        materials: [
            tierra_grama.clone(), tierra_grama.clone(), tierra.clone(), grama.clone(), tierra_grama.clone(), tierra_grama.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(2.0, 2.0, -4.0),  
        size: 1.0,                          
        materials: [
            tierra_grama.clone(), tierra_grama.clone(), tierra.clone(), grama.clone(), tierra_grama.clone(), tierra_grama.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(1.0, 2.0, -4.0),  
        size: 1.0,                          
        materials: [
            tierra_grama.clone(), tierra_grama.clone(), tierra.clone(), grama.clone(), tierra_grama.clone(), tierra_grama.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-1.0, 1.0, 3.0),  
        size: 1.0,                          
        materials: [
            piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-2.0, 1.0, 3.0),  
        size: 1.0,                          
        materials: [
            piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-3.0, 1.0, 3.0),  
        size: 1.0,                          
        materials: [
            piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-4.0, 1.0, 3.0),  
        size: 1.0,                          
        materials: [
            piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-4.0, 1.0, 2.0),  
        size: 1.0,                          
        materials: [
            piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-4.0, 1.0, 1.0),  
        size: 1.0,                          
        materials: [
            piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-4.0, 1.0, 0.0),  
        size: 1.0,                          
        materials: [
            piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-3.0, 1.0, 1.0),  
        size: 1.0,                          
        materials: [
            piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-2.0, 1.0, 2.0),  
        size: 1.0,                          
        materials: [
            piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-2.0, 2.0, 3.0),  
        size: 1.0,                          
        materials: [
            piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-3.0, 2.0, 3.0),  
        size: 1.0,                          
        materials: [
            piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-3.0, 2.0, 2.0),  
        size: 1.0,                          
        materials: [
            piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-4.0, 2.0, 3.0),  
        size: 1.0,                          
        materials: [
            piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-4.0, 2.0, 2.0),  
        size: 1.0,                          
        materials: [
            piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-4.0, 2.0, 1.0),  
        size: 1.0,                          
        materials: [
            piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-3.0, 3.0, 3.0),  
        size: 1.0,                          
        materials: [
            piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-4.0, 3.0, 3.0),  
        size: 1.0,                          
        materials: [
            piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-4.0, 3.0, 2.0),  
        size: 1.0,                          
        materials: [
            piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(2.0, 2.0, -2.0),  
        size: 1.0,                          
        materials: [
            madera.clone(), madera.clone(), madera.clone(), madera.clone(), madera.clone(), madera.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(2.0, 3.0, -2.0),  
        size: 1.0,                          
        materials: [
            madera.clone(), madera.clone(), madera.clone(), madera.clone(), madera.clone(), madera.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(2.0, 6.0, -2.0),  
        size: 1.0,                          
        materials: [
            hoja.clone(), hoja.clone(), hoja.clone(), hoja.clone(),hoja.clone(), hoja.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(2.0, 5.0, -1.0),  
        size: 1.0,                          
        materials: [
            hoja.clone(), hoja.clone(), hoja.clone(), hoja.clone(),hoja.clone(), hoja.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(2.0, 5.0, -3.0),  
        size: 1.0,                          
        materials: [
            hoja.clone(), hoja.clone(), hoja.clone(), hoja.clone(),hoja.clone(), hoja.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(3.0, 5.0, -2.0),  
        size: 1.0,                          
        materials: [
            hoja.clone(), hoja.clone(), hoja.clone(), hoja.clone(),hoja.clone(), hoja.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(1.0, 5.0, -2.0),  
        size: 1.0,                          
        materials: [
            hoja.clone(), hoja.clone(), hoja.clone(), hoja.clone(),hoja.clone(), hoja.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(2.0, 4.0, -1.0),  
        size: 1.0,                          
        materials: [
            hoja.clone(), hoja.clone(), hoja.clone(), hoja.clone(),hoja.clone(), hoja.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(2.0, 4.0, -3.0),  
        size: 1.0,                          
        materials: [
            hoja.clone(), hoja.clone(), hoja.clone(), hoja.clone(),hoja.clone(), hoja.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(3.0, 4.0, -2.0),  
        size: 1.0,                          
        materials: [
            hoja.clone(), hoja.clone(), hoja.clone(), hoja.clone(),hoja.clone(), hoja.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(1.0, 4.0, -2.0),  
        size: 1.0,                          
        materials: [
            hoja.clone(), hoja.clone(), hoja.clone(), hoja.clone(),hoja.clone(), hoja.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(3.0, 4.0, -1.0),  
        size: 1.0,                          
        materials: [
            hoja.clone(), hoja.clone(), hoja.clone(), hoja.clone(),hoja.clone(), hoja.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(1.0, 4.0, -1.0),  
        size: 1.0,                          
        materials: [
            hoja.clone(), hoja.clone(), hoja.clone(), hoja.clone(),hoja.clone(), hoja.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(1.0, 4.0, -3.0),  
        size: 1.0,                          
        materials: [
            hoja.clone(), hoja.clone(), hoja.clone(), hoja.clone(),hoja.clone(), hoja.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(3.0, 4.0, -3.0),  
        size: 1.0,                          
        materials: [
            hoja.clone(), hoja.clone(), hoja.clone(), hoja.clone(),hoja.clone(), hoja.clone(),
        ],
    }));

    // Ejemplo de un material transparente (por ejemplo, vidrio)
    let glass = Material::new(Color::new(255, 255, 255), 125.0, [0.0, 0.5, 0.1, 0.8], 1.5, false, None); // Vidrio, 80% transparente, índice de refracción 1.5
    
    let mut needs_render = true;
    let mut camera_moved = false;
    
    // Bucle principal para manejar la entrada del teclado y actualizar la cámara
    while window.is_open() && !window.is_key_down(Key::Escape) {
        camera_moved = false;

        // Handle camera movement
        if window.is_key_down(Key::W) {
            camera.mover_enfrente(0.2);
            camera_moved = true;
        }
        if window.is_key_down(Key::S) {
            camera.mover_atras(0.2);
            camera_moved = true;
        }
        if window.is_key_down(Key::A) {
            camera.mover_izq(0.2);
            camera_moved = true;
        }
        if window.is_key_down(Key::D) {
            camera.mover_der(0.2);
            camera_moved = true;
        }

        // Handle camera rotation
        if window.is_key_down(Key::Up) {
            camera.orbit(0.0, -0.05);
            camera_moved = true;
        }
        if window.is_key_down(Key::Down) {
            camera.orbit(0.0, 0.05);
            camera_moved = true;
        }
        if window.is_key_down(Key::Left) {
            camera.orbit(-0.05, 0.0);
            camera_moved = true;
        }
        if window.is_key_down(Key::Right) {
            camera.orbit(0.05, 0.0);
            camera_moved = true;
        }

        // Rendering
        if camera_moved {
            // Render at low resolution during movement
            render(&mut framebuffer_low, &objects, &camera, &lights);
            let scaled_framebuffer = upscale_framebuffer(framebuffer_low.get_buffer(), framebuffer_low.width, framebuffer_low.height, framebuffer_high.width, framebuffer_high.height);
            window.update_with_buffer(&scaled_framebuffer, framebuffer_high.width, framebuffer_high.height).unwrap();
        } else {
            // Render at high resolution when stationary
            render(&mut framebuffer_high, &objects, &camera, &lights);
            window.update_with_buffer(framebuffer_high.get_buffer(), framebuffer_high.width, framebuffer_high.height).unwrap();
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

fn upscale_framebuffer(low_res_buffer: &[u32], low_width: usize, low_height: usize, high_width: usize, high_height: usize) -> Vec<u32> {
    let mut high_res_buffer = vec![0; high_width * high_height];

    for y in 0..high_height {
        let src_y = y * low_height / high_height;
        for x in 0..high_width {
            let src_x = x * low_width / high_width;
            let src_index = src_y * low_width + src_x;
            let dst_index = y * high_width + x;
            high_res_buffer[dst_index] = low_res_buffer[src_index];
        }
    }

    high_res_buffer
}