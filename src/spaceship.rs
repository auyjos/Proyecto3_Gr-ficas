// spaceship.rs - Módulo para la nave espacial que sigue a la cámara

use crate::vertex::Vertex;
use crate::obj::Obj;
use crate::texture::Texture;
use raylib::prelude::*;

pub struct Spaceship {
    pub vertices: Vec<Vertex>,
    pub texture: Option<Texture>,
    pub orbit_radius: f32, // Radio de órbita alrededor del sistema
    pub orbit_speed: f32,  // Velocidad de órbita
    pub scale: f32,
    pub warp_offset: Vector3, // Offset adicional cuando está en warp mode
    pub is_warping: bool,     // Indica si está siguiendo un warp
}

impl Spaceship {
    pub fn new() -> Result<Self, String> {
        // Cargar el nuevo modelo OBJ de la nave
        let obj = Obj::load("assets/models/spaceship/SpaceShip.obj")
            .map_err(|e| format!("Error cargando nave: {:?}", e))?;
        let vertices = obj.get_vertex_array();
        let texture = obj.get_texture().clone();
        
        Ok(Spaceship {
            vertices,
            texture,
            // Orbita en el plano eclíptico, entre Marte y Júpiter
            orbit_radius: 180.0,
            orbit_speed: 0.15, // Velocidad moderada
            scale: 35.0, // Escala mucho mayor para que sea bien visible
            warp_offset: Vector3::zero(),
            is_warping: false,
        })
    }
    
    /// Calcula la posición de la nave basada en el tiempo (orbita circular)
    pub fn get_position(&self, time: f32, center: Vector3) -> Vector3 {
        let orbit_angle = time * self.orbit_speed;
        Vector3::new(
            center.x + orbit_angle.cos() * self.orbit_radius,
            center.y + orbit_angle.sin() * self.orbit_radius,
            center.z, // Mismo plano que los planetas
        )
    }
    
    /// Actualiza el estado de warp de la nave
    pub fn update_warp(&mut self, warp_offset: Vector3, is_warping: bool) {
        self.warp_offset = warp_offset;
        self.is_warping = is_warping;
    }
    
    /// Calcula la rotación de la nave para que mire hacia la dirección de movimiento
    pub fn get_rotation(&self, time: f32) -> Vector3 {
        let orbit_angle = time * self.orbit_speed;
        // Rotar en Y para que apunte en la dirección del movimiento
        Vector3::new(
            0.0,
            orbit_angle + std::f32::consts::PI / 2.0, // +90° para orientar correctamente
            0.0,
        )
    }
    
    /// Obtiene el array de vértices para renderizado
    pub fn get_vertices(&self) -> &[Vertex] {
        &self.vertices
    }
    
    /// Obtiene la textura de la nave
    pub fn get_texture(&self) -> &Option<Texture> {
        &self.texture
    }
}
