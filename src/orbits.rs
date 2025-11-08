// orbits.rs - Renderizado de órbitas planetarias

use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use std::f32::consts::PI;

/// Dibuja una órbita circular en el framebuffer
#[allow(dead_code)]
pub fn draw_orbit(
    framebuffer: &mut Framebuffer,
    center_x: f32,
    center_y: f32,
    radius: f32,
    color: Vector3,
    segments: usize,
) {
    let angle_step = (2.0 * PI) / segments as f32;
    
    for i in 0..segments {
        let angle1 = angle_step * i as f32;
        let angle2 = angle_step * ((i + 1) % segments) as f32;
        
        let x1 = center_x + radius * angle1.cos();
        let y1 = center_y + radius * angle1.sin();
        let x2 = center_x + radius * angle2.cos();
        let y2 = center_y + radius * angle2.sin();
        
        draw_line(framebuffer, x1, y1, x2, y2, color);
    }
}

/// Dibuja una línea entre dos puntos usando el algoritmo de Bresenham
#[allow(dead_code)]
fn draw_line(framebuffer: &mut Framebuffer, x0: f32, y0: f32, x1: f32, y1: f32, color: Vector3) {
    let mut x0 = x0 as i32;
    let mut y0 = y0 as i32;
    let x1 = x1 as i32;
    let y1 = y1 as i32;
    
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    
    loop {
        framebuffer.point(x0, y0, color);
        
        if x0 == x1 && y0 == y1 {
            break;
        }
        
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}
