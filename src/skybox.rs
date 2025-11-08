// skybox.rs - Sistema de estrellas de fondo

use raylib::prelude::*;
use rand::Rng;

/// Representa una estrella en el skybox
#[derive(Clone)]
pub struct Star {
    pub x: f32,
    pub y: f32,
    pub brightness: f32,
    pub twinkle_speed: f32,
}

pub struct Skybox {
    stars: Vec<Star>,
}

impl Skybox {
    /// Crea un nuevo skybox con estrellas aleatorias
    pub fn new(count: usize, width: u32, height: u32) -> Self {
        let mut rng = rand::thread_rng();
        let mut stars = Vec::with_capacity(count);
        
        for _ in 0..count {
            stars.push(Star {
                x: rng.gen_range(0.0..width as f32),
                y: rng.gen_range(0.0..height as f32),
                brightness: rng.gen_range(0.3..1.0),
                twinkle_speed: rng.gen_range(0.5..3.0),
            });
        }
        
        Self { stars }
    }
    
    /// Renderiza las estrellas en el framebuffer
    pub fn render(&self, framebuffer: &mut crate::framebuffer::Framebuffer, time: f32) {
        for star in &self.stars {
            // Efecto de parpadeo (twinkle)
            let twinkle = ((time * star.twinkle_speed).sin() * 0.5 + 0.5).max(0.0).min(1.0);
            let brightness = star.brightness * twinkle;
            
            // Color blanco con brillo variable
            let color = Vector3::new(brightness, brightness, brightness);
            
            // Dibujar la estrella (punto simple)
            framebuffer.point(star.x as i32, star.y as i32, color);
            
            // Para estrellas más brillantes, dibujar puntos adicionales
            if brightness > 0.7 {
                // Crear efecto de cruz pequeña
                framebuffer.point(star.x as i32 + 1, star.y as i32, color * 0.5);
                framebuffer.point(star.x as i32 - 1, star.y as i32, color * 0.5);
                framebuffer.point(star.x as i32, star.y as i32 + 1, color * 0.5);
                framebuffer.point(star.x as i32, star.y as i32 - 1, color * 0.5);
            }
        }
    }
}
