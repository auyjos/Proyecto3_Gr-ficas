// warp.rs - Sistema de teleportación instantánea (warping)

use raylib::prelude::*;

/// Representa un objetivo de warp
#[derive(Clone)]
pub struct WarpTarget {
    pub name: String,
    pub position: Vector3,
    pub zoom_level: f32,
}

/// Sistema de warp con animación
pub struct WarpSystem {
    pub targets: Vec<WarpTarget>,
    pub current_target: usize,
    pub is_warping: bool,
    pub warp_progress: f32,
    pub warp_duration: f32,
    pub start_offset: Vector3,
    pub start_zoom: f32,
    pub target_offset: Vector3,
    pub target_zoom: f32,
}

impl WarpSystem {
    pub fn new() -> Self {
        Self {
            targets: Vec::new(),
            current_target: 0,
            is_warping: false,
            warp_progress: 0.0,
            warp_duration: 1.0, // 1 segundo de animación
            start_offset: Vector3::zero(),
            start_zoom: 0.6,
            target_offset: Vector3::zero(),
            target_zoom: 0.6,
        }
    }
    
    /// Iniciar warp a un objetivo específico
    pub fn warp_to(
        &mut self,
        target_index: usize,
        current_offset: Vector3,
        current_zoom: f32,
    ) {
        if target_index < self.targets.len() && !self.is_warping {
            self.current_target = target_index;
            self.is_warping = true;
            self.warp_progress = 0.0;
            self.start_offset = current_offset;
            self.start_zoom = current_zoom;
            
            let target = &self.targets[target_index];
            self.target_offset = target.position;
            self.target_zoom = target.zoom_level;
        }
    }
    
    /// Actualizar animación de warp
    pub fn update(&mut self, delta_time: f32) -> (Vector3, f32) {
        if self.is_warping {
            self.warp_progress += delta_time / self.warp_duration;
            
            if self.warp_progress >= 1.0 {
                self.warp_progress = 1.0;
                self.is_warping = false;
            }
            
            // Ease-in-out usando función suave
            let t = self.ease_in_out_cubic(self.warp_progress);
            
            // Interpolar posición
            let offset = Vector3::new(
                self.start_offset.x + (self.target_offset.x - self.start_offset.x) * t,
                self.start_offset.y + (self.target_offset.y - self.start_offset.y) * t,
                self.start_offset.z + (self.target_offset.z - self.start_offset.z) * t,
            );
            
            // Interpolar zoom con efecto de "zoom out" en medio del viaje
            let zoom_mid = (self.start_zoom + self.target_zoom) * 0.3; // Zoom muy alejado en el medio
            let zoom = if t < 0.5 {
                // Primera mitad: zoom out
                let t1 = t * 2.0;
                self.start_zoom + (zoom_mid - self.start_zoom) * t1
            } else {
                // Segunda mitad: zoom in
                let t2 = (t - 0.5) * 2.0;
                zoom_mid + (self.target_zoom - zoom_mid) * t2
            };
            
            (offset, zoom)
        } else {
            (self.target_offset, self.target_zoom)
        }
    }
    
    /// Función de suavizado (ease-in-out cubic)
    fn ease_in_out_cubic(&self, t: f32) -> f32 {
        if t < 0.5 {
            4.0 * t * t * t
        } else {
            1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
        }
    }
    
    /// Obtener efecto visual de warp (para partículas, etc)
    pub fn get_warp_intensity(&self) -> f32 {
        if self.is_warping {
            // Pico de intensidad en el medio del warp
            let t = self.warp_progress;
            (t * (1.0 - t) * 4.0).min(1.0)
        } else {
            0.0
        }
    }
}

/// Calcular posición de destino para un planeta
#[allow(dead_code)]
pub fn calculate_planet_target(
    planet_orbit_radius: f32,
    _planet_scale: f32,
    center: Vector3,
    time: f32,
    orbit_speed: f32,
) -> Vector3 {
    let orbit_angle = time * orbit_speed;
    let orbit_x = orbit_angle.cos() * planet_orbit_radius;
    let orbit_y = orbit_angle.sin() * planet_orbit_radius;
    
    Vector3::new(
        orbit_x - center.x + 400.0, // Centrar en la ventana
        orbit_y - center.y + 300.0,
        0.0,
    )
}
