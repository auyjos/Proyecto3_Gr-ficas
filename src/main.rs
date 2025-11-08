// main.rs

mod framebuffer;
mod triangle;
mod line;
mod vertex;
mod fragment;
mod shaders;
mod obj;
mod matrix;
mod rings;
mod moons;
mod texture;
mod validate_textures;
mod solar_system;
mod skybox;
mod orbits;
mod warp;
mod spaceship;
mod collision;

use crate::matrix::new_matrix4;
use crate::shaders::get_planet_color;
use crate::texture::Texture;
use crate::solar_system::{create_solar_system, get_unique_model_paths};
use crate::skybox::Skybox;
use crate::warp::{WarpSystem, WarpTarget};
use crate::spaceship::Spaceship;
use crate::collision::check_spaceship_collisions;
use framebuffer::Framebuffer;
use vertex::Vertex;
use triangle::triangle;
use shaders::vertex_shader;
use obj::Obj;
use raylib::prelude::*;
use std::thread;
use std::time::Duration;

pub struct Uniforms {
    pub model_matrix: Matrix,
    pub time: f32,
    pub planet_type: u32,  // 0: Sun, 1: Earth-like, 2: Gas Giant, etc.
}

fn create_model_matrix(translation: Vector3, scale: f32, rotation: Vector3) -> Matrix {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    // Rotation around the X-axis
    let rotation_matrix_x = new_matrix4(
        1.0, 0.0,    0.0,    0.0,
        0.0, cos_x,  -sin_x, 0.0,
        0.0, sin_x,  cos_x,  0.0,
        0.0, 0.0,    0.0,    1.0
    );

    // Rotation around the Y-axis
    let rotation_matrix_y = new_matrix4(
        cos_y,  0.0, sin_y, 0.0,
        0.0,    1.0, 0.0,   0.0,
        -sin_y, 0.0, cos_y, 0.0,
        0.0,    0.0, 0.0,   1.0
    );

    // Rotation around the Z-axis
    let rotation_matrix_z = new_matrix4(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z, cos_z,  0.0, 0.0,
        0.0,   0.0,    1.0, 0.0,
        0.0,   0.0,    0.0, 1.0
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    // Scaling matrix
    let scale_matrix = new_matrix4(
        scale, 0.0,   0.0,   0.0,
        0.0,   scale, 0.0,   0.0,
        0.0,   0.0,   scale, 0.0,
        0.0,   0.0,   0.0,   1.0
    );

    // Translation matrix
    let translation_matrix = new_matrix4(
        1.0, 0.0, 0.0, translation.x,
        0.0, 1.0, 0.0, translation.y,
        0.0, 0.0, 1.0, translation.z,
        0.0, 0.0, 0.0, 1.0
    );

    scale_matrix * rotation_matrix * translation_matrix
}

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    // Vertex Shader Stage
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    // Primitive Assembly Stage
    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    // Rasterization Stage
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    // Fragment Processing Stage
    for fragment in fragments {
        // Create a temporary vertex at the fragment position for shader evaluation
        let temp_vertex = Vertex {
            position: Vector3::new(fragment.position.x, fragment.position.y, 0.0),
            normal: Vector3::new(0.0, 1.0, 0.0),
            tex_coords: Vector2::zero(),
            color: fragment.color, // Use material color from the vertex
            transformed_position: Vector3::new(fragment.position.x, fragment.position.y, fragment.depth),
            transformed_normal: Vector3::new(0.0, 1.0, 0.0),
        };
        
        // Apply shader to get color based on planet type
        let color = get_planet_color(&fragment, &temp_vertex, uniforms.time, uniforms.planet_type);
        
        framebuffer.point(
            fragment.position.x as i32,
            fragment.position.y as i32,
            color
        );
    }
}

fn main() {
    // Validar texturas antes de iniciar la aplicación
    validate_textures::validate_texture_paths();
    
    let window_width = 800;
    let window_height = 600;

    let (mut window, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Space Travel - Jose Auyon")
        .log_level(TraceLogLevel::LOG_WARNING) // Suppress INFO messages
        .build();

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);
    framebuffer.set_background_color(Vector3::new(0.01, 0.01, 0.05)); // Deep space black with slight blue tint

    // Initialize the texture inside the framebuffer
    framebuffer.init_texture(&mut window, &thread);

    // Animation parameters
    let mut time = 0.0f32;
    let mut auto_rotate = true;
    let mut auto_orbit = true;
    let mut show_controls = true; // Mostrar controles al inicio
    
    // Camera/viewport control
    let mut camera_offset = Vector3::new(0.0, 0.0, 0.0);
    let mut camera_zoom = 0.6f32;  // Start more zoomed out to see all planets
    let mut system_rotation = Vector3::new(0.0, 0.0, 0.0);

    // Load all unique models into a cache (with textures)
    let mut model_cache: std::collections::HashMap<String, (Vec<Vertex>, Option<Texture>)> = std::collections::HashMap::new();
    
    // Pre-load unique models
    let unique_models = get_unique_model_paths();
    
    // Validar carga de texturas en runtime
    println!("\n=== CARGANDO MODELOS ===\n");
    
    for model_path in unique_models {
        match Obj::load(model_path) {
            Ok(obj) => {
                let vertex_array = obj.get_vertex_array();
                let texture = obj.get_texture().clone();
                
                // Verificar si la textura se cargó
                if texture.is_some() {
                    println!("✓ Modelo cargado con textura: {}", model_path);
                } else {
                    println!("⚠ Modelo cargado SIN textura: {}", model_path);
                }
                
                model_cache.insert(model_path.to_string(), (vertex_array, texture));
            }
            Err(e) => {
                eprintln!("✗ Error al cargar {}: {:?}", model_path, e);
            }
        }
    }
    
    println!("\n=== RESUMEN DE CARGA ===");
    println!("Total de modelos únicos: {}", model_cache.len());
    println!("Modelos con textura: {}", model_cache.values().filter(|(_, tex)| tex.is_some()).count());
    println!();

    // Crear el sistema solar
    let bodies = create_solar_system();
    
    // Crear skybox con estrellas
    let skybox = Skybox::new(300, window_width as u32, window_height as u32);
    
    // Crear la nave espacial
    let mut spaceship = match Spaceship::new() {
        Ok(ship) => {
            println!("✓ Nave espacial cargada exitosamente (nuevo modelo SpaceShip.obj)");
            println!("  - Vértices: {}", ship.get_vertices().len());
            println!("  - Textura: {}", if ship.get_texture().is_some() { "Sí" } else { "No (usa materiales de color)" });
            Some(ship)
        }
        Err(e) => {
            eprintln!("⚠ No se pudo cargar la nave: {}", e);
            None
        }
    };
    
    // Crear sistema de warp
    let mut warp_system = WarpSystem::new();
    
    // Configurar objetivos de warp para cada planeta
    for (i, body) in bodies.iter().enumerate() {
        warp_system.targets.push(WarpTarget {
            name: body.name.clone(),
            position: Vector3::zero(), // Se calculará dinámicamente
            zoom_level: if i == 0 { 1.2 } else { 2.0 }, // Sol más alejado, planetas más cerca
        });
    }

    while !window.window_should_close() {
        handle_input(&mut window, &mut camera_offset, &mut camera_zoom, &mut system_rotation, &mut auto_rotate, &mut auto_orbit, &mut show_controls, &mut warp_system, &bodies, time);

        // Update time
        time += 0.016; // Approximately 60 FPS
        
        // Actualizar warp system
        if warp_system.is_warping {
            // Recalcular la posición del planeta destino en tiempo real
            let target_index = warp_system.current_target;
            if target_index < bodies.len() {
                let body = &bodies[target_index];
                let orbit_angle = time * body.orbit_speed;
                let target_x = orbit_angle.cos() * body.orbit_radius;
                let target_y = orbit_angle.sin() * body.orbit_radius;
                
                // Actualizar el target con la posición actual del planeta (como offset de cámara)
                let target_zoom = warp_system.targets[target_index].zoom_level;
                warp_system.targets[target_index].position = Vector3::new(-target_x * target_zoom, -target_y * target_zoom, 0.0);
            }
            
            let (new_offset, new_zoom) = warp_system.update(0.016);
            camera_offset = new_offset;
            camera_zoom = new_zoom;
        }

        framebuffer.clear();
        
        // Renderizar skybox primero (fondo de estrellas)
        skybox.render(&mut framebuffer, time);
        
        // Efecto visual de warp - anillos pulsantes
        if warp_system.is_warping {
            let intensity = warp_system.get_warp_intensity();
            let center_x = (window_width / 2) as i32;
            let center_y = (window_height / 2) as i32;
            
            // Dibujar anillos pulsantes desde el centro
            for ring in 0..5 {
                let radius = (50 + ring * 80) as f32 * intensity;
                let brightness = (1.0 - ring as f32 * 0.15) * (1.0 - intensity) * 0.5;
                let color = Vector3::new(
                    brightness * 0.5,
                    brightness * 0.7,
                    brightness
                );
                
                // Dibujar círculo aproximado con puntos
                for angle in (0..360).step_by(3) {
                    let rad = (angle as f32).to_radians();
                    let x = center_x + (rad.cos() * radius) as i32;
                    let y = center_y + (rad.sin() * radius) as i32;
                    framebuffer.point(x, y, color);
                }
            }
        }

        // Center point for the solar system (affected by camera X/Y offset only)
        let center = Vector3::new(400.0 + camera_offset.x, 300.0 + camera_offset.y, 0.0);
        
        // Renderizar órbitas planetarias (círculos rotados en 3D)
        for body in bodies.iter() {
            if body.orbit_radius > 0.0 {
                let orbit_color = Vector3::new(0.4, 0.5, 0.7); // Color azul brillante
                let segments = 200; // Más segmentos para líneas continuas
                
                // Generar puntos de la órbita circular y rotarlos en 3D
                for i in 0..segments {
                    let angle = (i as f32 / segments as f32) * 2.0 * std::f32::consts::PI;
                    
                    // Punto en el plano eclíptico 3D (antes de rotar)
                    let orbit_point_3d = Vector3::new(
                        angle.cos() * body.orbit_radius * camera_zoom,
                        angle.sin() * body.orbit_radius * camera_zoom,
                        0.0
                    );
                    
                    // Aplicar rotación 3D del sistema
                    let rotated_point = rotate_point_around_center(orbit_point_3d, Vector3::zero(), system_rotation);
                    
                    // Proyectar a 2D
                    let x = (center.x + rotated_point.x) as i32;
                    let y = (center.y + rotated_point.y) as i32;
                    
                    // Dibujar el punto de la órbita con grosor (3x3 píxeles)
                    for dx in -1..=1 {
                        for dy in -1..=1 {
                            let px = x + dx;
                            let py = y + dy;
                            if px >= 0 && px < framebuffer.width as i32 &&
                               py >= 0 && py < framebuffer.height as i32 {
                                framebuffer.point(px, py, orbit_color);
                            }
                        }
                    }
                }
            }
        }
        
        // Renderizar órbita de la nave espacial (rotada en 3D)
        if let Some(ref ship) = spaceship {
            let ship_orbit_color = Vector3::new(0.6, 0.8, 0.4); // Verde claro
            let segments = 200;
            
            for i in 0..segments {
                let angle = (i as f32 / segments as f32) * 2.0 * std::f32::consts::PI;
                
                // Punto orbital 3D antes de rotar
                let orbit_point_3d = Vector3::new(
                    angle.cos() * ship.orbit_radius * camera_zoom,
                    angle.sin() * ship.orbit_radius * camera_zoom,
                    0.0
                );
                
                // Aplicar rotación 3D del sistema
                let rotated_point = rotate_point_around_center(orbit_point_3d, Vector3::zero(), system_rotation);
                
                let x = (center.x + rotated_point.x) as i32;
                let y = (center.y + rotated_point.y) as i32;
                
                // Dibujar órbita de la nave con línea punteada (más sutil)
                if i % 4 == 0 {
                    for dx in -1..=1 {
                        for dy in -1..=1 {
                            let px = x + dx;
                            let py = y + dy;
                            if px >= 0 && px < framebuffer.width as i32 &&
                               py >= 0 && py < framebuffer.height as i32 {
                                framebuffer.point(px, py, ship_orbit_color);
                            }
                        }
                    }
                }
            }
        }

        // Render all celestial bodies
        for body in bodies.iter() {
            // Calculate position
            let body_rotation = if auto_rotate {
                Vector3::new(0.0, time * body.rotation_speed, 0.0)
            } else {
                Vector3::new(0.0, 0.0, 0.0)
            };

            let body_translation = if auto_orbit {
                let orbit_angle = time * body.orbit_speed;
                
                // Posición orbital en 3D (antes de rotar el sistema)
                let orbit_point_3d = Vector3::new(
                    orbit_angle.cos() * body.orbit_radius * camera_zoom,
                    orbit_angle.sin() * body.orbit_radius * camera_zoom,
                    0.0
                );
                
                // Aplicar rotación 3D del sistema completo
                let rotated_point = rotate_point_around_center(orbit_point_3d, Vector3::zero(), system_rotation);
                
                Vector3::new(
                    center.x + rotated_point.x,
                    center.y + rotated_point.y,
                    center.z + rotated_point.z,
                )
            } else {
                center
            };

            // Crear model matrix con escala aplicada
            let model_matrix = create_model_matrix(body_translation, body.scale * camera_zoom, body_rotation);
            let uniforms = Uniforms {
                model_matrix,
                time,
                planet_type: body.planet_type(),
            };

            // Get the vertex array for this body's model
            if let Some((vertex_array, _texture)) = model_cache.get(&body.model_path) {
                render(&mut framebuffer, &uniforms, vertex_array);
            }

            // Renderizar lunas si el planeta las tiene
            for moon in &body.moons {
                let moon_orbit_angle = time * moon.orbit_speed;
                let moon_pos = Vector3::new(
                    body_translation.x + moon_orbit_angle.cos() * moon.orbit_radius,
                    body_translation.y + moon_orbit_angle.sin() * moon.orbit_radius,
                    body_translation.z,
                );
                
                let moon_rotation = if auto_rotate {
                    Vector3::new(0.0, time * moon.rotation_speed, 0.0)
                } else {
                    Vector3::new(0.0, 0.0, 0.0)
                };
                
                let moon_matrix = create_model_matrix(moon_pos, moon.scale * camera_zoom, moon_rotation);
                let moon_uniforms = Uniforms {
                    model_matrix: moon_matrix,
                    time,
                    planet_type: 3, // Moon shader
                };
                
                // Use moon model with 2k_moon texture
                if let Some((moon_vertex_array, _moon_texture)) = model_cache.get("assets/models/moon.obj") {
                    render(&mut framebuffer, &moon_uniforms, moon_vertex_array);
                }
            }
            
            // Renderizar anillos si el planeta los tiene
            if body.has_rings {
                let ring_scale = 1.8;
                let ring_matrix = create_model_matrix(body_translation, body.scale * ring_scale * camera_zoom, Vector3::new(0.2, 0.0, 0.0));
                let ring_uniforms = Uniforms {
                    model_matrix: ring_matrix,
                    time,
                    planet_type: 4, // Ring shader
                };
                
                // Generate and render ring geometry
                let ring_vertices = rings::generate_flat_ring(1.0, 1.5, 128);
                render(&mut framebuffer, &ring_uniforms, &ring_vertices);
            }
        }

        // Renderizar la nave espacial (orbita en el plano eclíptico rotado)
        let mut collision_detected = false;
        let mut collision_planet_idx: Option<usize> = None;
        
        if let Some(ref mut ship) = spaceship {
            // Actualizar estado de warp de la nave (solo para tracking, no afecta posición)
            ship.update_warp(camera_offset, warp_system.is_warping);
            
            // Calcular posición orbital 3D de la nave
            // IMPORTANTE: center ya incluye camera_offset, así que la nave se mueve automáticamente con el warp
            let orbit_angle_ship = time * ship.orbit_speed;
            let ship_orbit_3d = Vector3::new(
                orbit_angle_ship.cos() * ship.orbit_radius * camera_zoom,
                orbit_angle_ship.sin() * ship.orbit_radius * camera_zoom,
                0.0
            );
            
            // Aplicar rotación 3D del sistema
            let rotated_ship_orbit = rotate_point_around_center(ship_orbit_3d, Vector3::zero(), system_rotation);
            
            // La posición usa center, que ya incluye camera_offset (por lo tanto, ya incluye el warp)
            let ship_position = Vector3::new(
                center.x + rotated_ship_orbit.x,
                center.y + rotated_ship_orbit.y,
                center.z + rotated_ship_orbit.z
            );
            
            let ship_rotation = ship.get_rotation(time);
            
            // Check for collisions with planets (usar posiciones rotadas)
            let mut planet_positions: Vec<(Vector3, f32)> = Vec::new();
            for body in bodies.iter() {
                let orbit_angle = if auto_orbit {
                    time * body.orbit_speed
                } else {
                    0.0
                };
                let orbit_point_3d = Vector3::new(
                    orbit_angle.cos() * body.orbit_radius * camera_zoom,
                    orbit_angle.sin() * body.orbit_radius * camera_zoom,
                    0.0
                );
                let rotated_orbit = rotate_point_around_center(orbit_point_3d, Vector3::zero(), system_rotation);
                let planet_pos = Vector3::new(center.x + rotated_orbit.x, center.y + rotated_orbit.y, center.z + rotated_orbit.z);
                planet_positions.push((planet_pos, body.scale * camera_zoom));
            }
            
            // Detect collisions
            if let Some(idx) = check_spaceship_collisions(ship_position, ship.scale * camera_zoom * 0.5, &planet_positions) {
                collision_detected = true;
                collision_planet_idx = Some(idx);
            }
            
            let ship_matrix = create_model_matrix(ship_position, ship.scale * camera_zoom, ship_rotation);
            let ship_uniforms = Uniforms {
                model_matrix: ship_matrix,
                time,
                planet_type: 10, // Tipo especial para la nave
            };
            
            render(&mut framebuffer, &ship_uniforms, ship.get_vertices());
        }

        // Display framebuffer and text overlay
        framebuffer.update_texture();
        
        let mut draw_handle = window.begin_drawing(&thread);
        draw_handle.clear_background(Color::BLACK);
        framebuffer.draw(&mut draw_handle);
        
        // UI Minimalista - Solo información esencial
        
        // Top-right: FPS (pequeño y discreto)
        let fps_text = format!("{}", draw_handle.get_fps());
        draw_handle.draw_text(&fps_text, window_width as i32 - 40, 10, 14, Color::new(150, 150, 150, 180));
        
        // Center: Solo mostrar durante warp
        if warp_system.is_warping {
            let target_name = &warp_system.targets[warp_system.current_target].name;
            let warp_text = format!("{}", target_name);
            let warp_intensity = warp_system.get_warp_intensity();
            let alpha = (200.0 * (1.0 - warp_intensity.abs())) as u8;
            
            // Centrar texto aproximadamente
            let text_x = (window_width as i32 / 2) - (target_name.len() as i32 * 10);
            draw_handle.draw_text(
                &warp_text,
                text_x.max(50),
                (window_height as i32) / 2 - 50,
                32,
                Color::new(200, 200, 255, alpha)
            );
        }
        
        // Collision warning (if detected)
        if collision_detected {
            if let Some(idx) = collision_planet_idx {
                let planet_name = &bodies[idx].name;
                let warning_text = format!("⚠ COLLISION: {}", planet_name);
                let text_width = (warning_text.len() * 10) as i32;
                draw_handle.draw_rectangle(
                    (window_width as i32 / 2) - (text_width / 2) - 10,
                    100,
                    text_width + 20,
                    30,
                    Color::new(255, 50, 50, 200)
                );
                draw_handle.draw_text(
                    &warning_text,
                    (window_width as i32 / 2) - (text_width / 2),
                    107,
                    18,
                    Color::WHITE
                );
            }
        }
        
        // Bottom-left: Indicadores mínimos
        let bottom_y = window_height as i32 - 30;
        let icon_inactive = Color::new(100, 100, 100, 150);
        let icon_active = Color::new(100, 200, 255, 255);
        
        // Rotación - R
        let rot_color = if auto_rotate { icon_active } else { icon_inactive };
        draw_handle.draw_text("R", 15, bottom_y, 18, rot_color);
        
        // Órbita - O
        let orb_color = if auto_orbit { icon_active } else { icon_inactive };
        draw_handle.draw_text("O", 40, bottom_y, 18, orb_color);
        
        // Zoom (restaurado)
        draw_handle.draw_text(&format!("{:.1}x", camera_zoom), 65, bottom_y, 16, Color::new(150, 150, 150, 180));
        
        // Bottom-right: Help hint
        draw_handle.draw_text("H", window_width as i32 - 30, bottom_y, 16, Color::new(150, 150, 150, 180));
        
        // Mostrar controles si está activado
        if show_controls {
            let panel_x = 20;
            let panel_y = 80;
            let line_height = 22;
            let text_color = Color::new(220, 220, 220, 255);
            let title_color = Color::new(100, 200, 255, 255);
            
            // Fondo semi-transparente
            draw_handle.draw_rectangle(panel_x - 10, panel_y - 10, 300, 192, Color::new(0, 0, 0, 180));
            
            // Título
            draw_handle.draw_text("CONTROLES", panel_x, panel_y, 20, title_color);
            
            let mut y = panel_y + 30;
            draw_handle.draw_text("SPACE    Pausar/Reanudar rotacion", panel_x, y, 14, text_color);
            y += line_height;
            draw_handle.draw_text("O        Pausar/Reanudar orbita", panel_x, y, 14, text_color);
            y += line_height;
            draw_handle.draw_text("Flechas  Mover camara (X/Y)", panel_x, y, 14, text_color);
            y += line_height;
            draw_handle.draw_text("W / Q    Mover camara (Z)", panel_x, y, 14, text_color);
            y += line_height;
            draw_handle.draw_text("S / A    Zoom In / Out", panel_x, y, 14, text_color);
            y += line_height;
            draw_handle.draw_text("1-9      Warp a planeta", panel_x, y, 14, text_color);
            y += line_height + 5;
            draw_handle.draw_text("H        Mostrar/Ocultar ayuda", panel_x, y, 14, title_color);
        }

        thread::sleep(Duration::from_millis(16));
    }
}

// Helper function to rotate a point around a center point
fn rotate_point_around_center(point: Vector3, center: Vector3, rotation: Vector3) -> Vector3 {
    // Translate to origin
    let p = Vector3::new(point.x - center.x, point.y - center.y, point.z - center.z);
    
    // Apply rotation matrices
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();
    
    // Rotate around X
    let p = Vector3::new(p.x, p.y * cos_x - p.z * sin_x, p.y * sin_x + p.z * cos_x);
    
    // Rotate around Y
    let p = Vector3::new(p.x * cos_y + p.z * sin_y, p.y, -p.x * sin_y + p.z * cos_y);
    
    // Rotate around Z
    let p = Vector3::new(p.x * cos_z - p.y * sin_z, p.x * sin_z + p.y * cos_z, p.z);
    
    // Translate back
    Vector3::new(p.x + center.x, p.y + center.y, p.z + center.z)
}

fn handle_input(
    window: &mut RaylibHandle,
    camera_offset: &mut Vector3,
    camera_zoom: &mut f32,
    system_rotation: &mut Vector3,
    auto_rotate: &mut bool,
    auto_orbit: &mut bool,
    show_controls: &mut bool,
    warp_system: &mut WarpSystem,
    bodies: &[solar_system::CelestialBody],
    time: f32,
) {
    // Camera movement 3D (arrow keys for X/Y, W/Q for Z-axis)
    if window.is_key_down(KeyboardKey::KEY_RIGHT) {
        camera_offset.x += 10.0;
    }
    if window.is_key_down(KeyboardKey::KEY_LEFT) {
        camera_offset.x -= 10.0;
    }
    if window.is_key_down(KeyboardKey::KEY_UP) {
        camera_offset.y -= 10.0;
    }
    if window.is_key_down(KeyboardKey::KEY_DOWN) {
        camera_offset.y += 10.0;
    }
    
    // 3D Rotation of the entire system (W/Q for X-axis rotation)
    if window.is_key_down(KeyboardKey::KEY_W) {
        system_rotation.x += 0.02;  // Rotate system down (view from above)
    }
    if window.is_key_down(KeyboardKey::KEY_Q) {
        system_rotation.x -= 0.02;  // Rotate system up (view from below)
    }
    
    // Zoom (S/A keys) - RESTAURADO
    if window.is_key_down(KeyboardKey::KEY_S) {
        *camera_zoom += 0.05;
        if *camera_zoom > 3.0 { *camera_zoom = 3.0; }
    }
    if window.is_key_down(KeyboardKey::KEY_A) {
        *camera_zoom -= 0.05;
        if *camera_zoom < 0.3 { *camera_zoom = 0.3; }
    }
    
    // Toggle auto-rotation with SPACE
    if window.is_key_pressed(KeyboardKey::KEY_SPACE) {
        *auto_rotate = !*auto_rotate;
    }
    
    // Toggle auto-orbit with O
    if window.is_key_pressed(KeyboardKey::KEY_O) {
        *auto_orbit = !*auto_orbit;
    }
    
    // Toggle controls display with H
    if window.is_key_pressed(KeyboardKey::KEY_H) {
        *show_controls = !*show_controls;
    }
    
    // Instant Warp con teclas numéricas (1-9)
    let warp_keys = [
        (KeyboardKey::KEY_ONE, 0),    // 1 - Sol
        (KeyboardKey::KEY_TWO, 1),    // 2 - Mercurio
        (KeyboardKey::KEY_THREE, 2),  // 3 - Venus
        (KeyboardKey::KEY_FOUR, 3),   // 4 - Tierra
        (KeyboardKey::KEY_FIVE, 4),   // 5 - Marte
        (KeyboardKey::KEY_SIX, 5),    // 6 - Júpiter
        (KeyboardKey::KEY_SEVEN, 6),  // 7 - Saturno
        (KeyboardKey::KEY_EIGHT, 7),  // 8 - Urano
        (KeyboardKey::KEY_NINE, 8),   // 9 - Neptuno
    ];
    
    for (key, index) in warp_keys.iter() {
        if window.is_key_pressed(*key) && *index < bodies.len() {
            // Calcular posición del planeta en este momento
            let body = &bodies[*index];
            let orbit_angle = time * body.orbit_speed;
            let target_x = orbit_angle.cos() * body.orbit_radius;
            let target_y = orbit_angle.sin() * body.orbit_radius;
            
            // Calcular el offset de cámara objetivo que centra el planeta en pantalla.
            // Queremos que: center_final.x + orbit_x * target_zoom = screen_center_x (400)
            // Dado center_final.x = 400 + target_camera_offset.x -> target_camera_offset.x = - orbit_x * target_zoom
            let target_zoom = warp_system.targets[*index].zoom_level;
            let target_offset = Vector3::new(-target_x * target_zoom, -target_y * target_zoom, 0.0);

            // Actualizar el target de warp con el offset de cámara corregido
            warp_system.targets[*index].position = target_offset;

            // Iniciar warp usando el offset y el zoom actuales
            warp_system.warp_to(*index, *camera_offset, *camera_zoom);
        }
    }
}
