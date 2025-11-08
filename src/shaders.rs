use raylib::prelude::*;
use crate::vertex::Vertex;
use crate::fragment::Fragment;
use crate::Uniforms;

// This function manually multiplies a 4x4 matrix with a 4D vector (in homogeneous coordinates)
fn multiply_matrix_vector4(matrix: &Matrix, vector: &Vector4) -> Vector4 {
    Vector4::new(
        matrix.m0 * vector.x + matrix.m4 * vector.y + matrix.m8 * vector.z + matrix.m12 * vector.w,
        matrix.m1 * vector.x + matrix.m5 * vector.y + matrix.m9 * vector.z + matrix.m13 * vector.w,
        matrix.m2 * vector.x + matrix.m6 * vector.y + matrix.m10 * vector.z + matrix.m14 * vector.w,
        matrix.m3 * vector.x + matrix.m7 * vector.y + matrix.m11 * vector.z + matrix.m15 * vector.w,
    )
}

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
  // Convert vertex position to homogeneous coordinates (Vec4) by adding a w-component of 1.0
  let position_vec4 = Vector4::new(
    vertex.position.x,
    vertex.position.y,
    vertex.position.z,
    1.0
  );

  // Apply the transformation by multiplying the model matrix with the vector
  let transformed_vec4 = multiply_matrix_vector4(&uniforms.model_matrix, &position_vec4);

  // Perform perspective division to convert from homogeneous coordinates back to 3D Cartesian coordinates
  let transformed_position_3d = if transformed_vec4.w != 0.0 {
      Vector3::new(
          transformed_vec4.x / transformed_vec4.w,
          transformed_vec4.y / transformed_vec4.w,
          transformed_vec4.z / transformed_vec4.w,
      )
  } else {
      // Avoid division by zero, though w should usually be 1 for model transformations
      Vector3::new(transformed_vec4.x, transformed_vec4.y, transformed_vec4.z)
  };

  // Simple isometric projection with subtle Z effect
  // The Z position slightly affects Y and scale but doesn't distort too much
  let z_factor = transformed_position_3d.z * 0.02;
  
  let transformed_position = Vector3::new(
      transformed_position_3d.x,
      transformed_position_3d.y + z_factor * 5.0, // Slight vertical shift for depth
      transformed_position_3d.z,
  );

  // Create a new Vertex with the transformed position
  Vertex {
    position: vertex.position,
    normal: vertex.normal,
    tex_coords: vertex.tex_coords,
    color: vertex.color,
    transformed_position,
    transformed_normal: vertex.normal, // Note: Correct normal transformation is more complex
  }
}

// ==================== FRAGMENT SHADERS ====================

/// Simple hash function for noise generation
fn hash(x: f32) -> f32 {
    let h = (x * 43758.5453).sin();
    h - h.floor()
}

/// 2D noise function
fn noise(p: Vector2) -> f32 {
    let i = Vector2::new(p.x.floor(), p.y.floor());
    let f = Vector2::new(p.x - i.x, p.y - i.y);
    
    // Four corners
    let a = hash(i.x + i.y * 57.0);
    let b = hash(i.x + 1.0 + i.y * 57.0);
    let c = hash(i.x + (i.y + 1.0) * 57.0);
    let d = hash(i.x + 1.0 + (i.y + 1.0) * 57.0);
    
    // Smooth interpolation
    let u = f.x * f.x * (3.0 - 2.0 * f.x);
    let v = f.y * f.y * (3.0 - 2.0 * f.y);
    
    let result = a * (1.0 - u) * (1.0 - v) +
                 b * u * (1.0 - v) +
                 c * (1.0 - u) * v +
                 d * u * v;
    result
}

/// Fractal Brownian Motion (FBM) for natural patterns
fn fbm(p: Vector2, octaves: i32) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 0.5;
    let mut frequency = 1.0;
    let mut max_value = 0.0;
    
    for _ in 0..octaves {
        value += amplitude * noise(p * frequency);
        max_value += amplitude;
        amplitude *= 0.5;
        frequency *= 2.0;
    }
    
    value / max_value
}

/// Mix/Lerp two colors
fn mix_color(a: Vector3, b: Vector3, t: f32) -> Vector3 {
    Vector3::new(
        a.x * (1.0 - t) + b.x * t,
        a.y * (1.0 - t) + b.y * t,
        a.z * (1.0 - t) + b.z * t,
    )
}

/// Smooth step function
fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Calculate lighting based on normal and light direction
fn calculate_lighting(normal: Vector3, light_dir: Vector3, view_dir: Vector3) -> f32 {
    // Normalize vectors
    let n = normalize(normal);
    let l = normalize(light_dir);
    let v = normalize(view_dir);
    
    // Diffuse lighting (Lambertian)
    let diffuse = (n.x * l.x + n.y * l.y + n.z * l.z).max(0.0);
    
    // Specular lighting (Phong)
    let r = reflect(l, n);
    let specular = (r.x * v.x + r.y * v.y + r.z * v.z).max(0.0).powf(32.0);
    
    // Ambient + Diffuse + Specular
    0.2 + diffuse * 0.7 + specular * 0.3
}

/// Normalize a vector
fn normalize(v: Vector3) -> Vector3 {
    let len = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
    if len > 0.0001 {
        Vector3::new(v.x / len, v.y / len, v.z / len)
    } else {
        v
    }
}

/// Reflect vector v around normal n
fn reflect(v: Vector3, n: Vector3) -> Vector3 {
    let dot2 = 2.0 * (v.x * n.x + v.y * n.y + v.z * n.z);
    Vector3::new(
        v.x - dot2 * n.x,
        v.y - dot2 * n.y,
        v.z - dot2 * n.z,
    )
}

/// SUN SHADER - Simply shows the texture with a subtle glow
fn sun_shader(_fragment: &Fragment, vertex: &Vertex, time: f32) -> Vector3 {
    // Just return the texture color with a subtle animated glow
    let texture_color = vertex.color;
    let glow = 1.0 + (time * 0.5).sin() * 0.15;
    
    Vector3::new(
        texture_color.x * glow,
        texture_color.y * glow,
        texture_color.z * glow,
    )
}

/// EARTH-LIKE PLANET - Simply shows the texture
fn earth_shader(_fragment: &Fragment, vertex: &Vertex, _time: f32) -> Vector3 {
    // Just return the texture color directly
    vertex.color
}

/// GAS GIANT - Shows texture with subtle atmospheric effects
fn gas_giant_shader(_fragment: &Fragment, vertex: &Vertex, time: f32) -> Vector3 {
    // Get texture color directly from vertex (already sampled from texture in obj.rs)
    let texture_color = vertex.color;
    
    // UV coordinates from position (for subtle effects only)
    let pos = vertex.transformed_position;
    let len = (pos.x * pos.x + pos.y * pos.y + pos.z * pos.z).sqrt();
    if len < 0.001 {
        return texture_color;
    }
    
    let norm = Vector3::new(pos.x / len, pos.y / len, pos.z / len);
    let u = (norm.x.atan2(norm.z) / std::f32::consts::PI + 1.0) * 0.5;
    let v = (norm.y).asin() / std::f32::consts::PI + 0.5;
    
    // SUBTLE atmospheric animation (very light)
    let atmosphere_noise = fbm(Vector2::new(u * 4.0, v * 4.0 + time * 0.05), 2);
    let atmosphere_factor = atmosphere_noise * 0.1; // Only 10% effect
    
    // Blend texture with very subtle atmospheric variation
    Vector3::new(
        texture_color.x * (1.0 + atmosphere_factor),
        texture_color.y * (1.0 + atmosphere_factor),
        texture_color.z * (1.0 + atmosphere_factor),
    )
}

/// MOON SHADER - Simply shows the texture
fn moon_shader(_fragment: &Fragment, vertex: &Vertex, _time: f32) -> Vector3 {
    // Just return the texture color directly
    vertex.color
}

/// RING SHADER - Saturn-like rings with bands
fn ring_shader(_fragment: &Fragment, vertex: &Vertex, time: f32) -> Vector3 {
    let u = vertex.tex_coords.x;
    let v = vertex.tex_coords.y;
    
    // Distance from center (0 = inner, 1 = outer)
    let r = (u - 0.5) * (u - 0.5) + (v - 0.5) * (v - 0.5);
    let r = r.sqrt() * 2.0;
    
    // Layer 1: Base ring color (pale gold)
    let base = Vector3::new(0.9, 0.85, 0.6);
    
    // Layer 2: Ring bands (alternating darker and lighter bands)
    let bands = ((r * 30.0).sin() * 0.5 + 0.5).max(0.0).min(1.0);
    let band_color = Vector3::new(0.7, 0.6, 0.3);
    let with_bands = mix_color(base, band_color, bands * 0.5);
    
    // Layer 3: Particle shadows
    let particles = fbm(Vector2::new(u * 10.0, r * 20.0 + time * 0.5), 3);
    let shadow = mix_color(with_bands, Vector3::new(0.5, 0.4, 0.1), particles * 0.4);
    
    // Layer 4: Edge darker (depth effect)
    let edge_darkness = smoothstep(0.0, 0.2, r) * smoothstep(1.5, 1.0, r);
    let result = mix_color(shadow, Vector3::new(0.2, 0.15, 0.05), (1.0 - edge_darkness) * 0.6);
    
    result
}

/// NEPTUNE - Simply shows the texture
fn neptune_shader(_fragment: &Fragment, vertex: &Vertex, _time: f32) -> Vector3 {
    // Just return the texture color directly
    vertex.color
}

/// URANUS - Simply shows the texture
fn uranus_shader(_fragment: &Fragment, vertex: &Vertex, _time: f32) -> Vector3 {
    // Just return the texture color directly
    vertex.color
}

/// VENUS - Simply shows the texture
fn venus_shader(_fragment: &Fragment, vertex: &Vertex, _time: f32) -> Vector3 {
    // Just return the texture color directly
    vertex.color
}

/// SPACESHIP - Metallic shader con colores del material
fn spaceship_shader(_fragment: &Fragment, vertex: &Vertex, time: f32) -> Vector3 {
    // Get base color del material (desde el OBJ)
    let base_color = vertex.color;
    
    // Efecto metálico - variación sutil de brillo
    let shimmer = (time * 1.5).sin() * 0.08 + 1.0;
    
    // Efecto de iluminación ambiental
    let ambient = 0.3;
    let diffuse = 0.7;
    
    // Simular iluminación direccional simple
    let light_factor = ambient + diffuse;
    
    // Combinar con shimmer metálico
    Vector3::new(
        (base_color.x * light_factor * shimmer).min(1.0),
        (base_color.y * light_factor * shimmer).min(1.0),
        (base_color.z * light_factor * shimmer).min(1.0),
    )
}

/// Get the appropriate shader color based on planet type
pub fn get_planet_color(fragment: &Fragment, vertex: &Vertex, time: f32, planet_type: u32) -> Vector3 {
    // Get base color from shader - NO LIGHTING, just pure textures
    match planet_type {
        0 => sun_shader(fragment, vertex, time),
        1 => earth_shader(fragment, vertex, time),
        2 => gas_giant_shader(fragment, vertex, time),
        3 => moon_shader(fragment, vertex, time),    // Moon shader
        4 => ring_shader(fragment, vertex, time),    // Ring shader
        5 => neptune_shader(fragment, vertex, time), // Neptune shader
        6 => uranus_shader(fragment, vertex, time),  // Uranus shader
        7 => venus_shader(fragment, vertex, time),   // Venus shader
        10 => spaceship_shader(fragment, vertex, time), // Spaceship shader
        _ => Vector3::new(1.0, 1.0, 1.0), // Default white
    }
}