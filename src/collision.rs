use raylib::math::Vector3;

/// Verifica colisión entre dos esferas usando distancia euclidiana
pub fn check_sphere_collision(pos1: Vector3, radius1: f32, pos2: Vector3, radius2: f32) -> bool {
    let dx = pos1.x - pos2.x;
    let dy = pos1.y - pos2.y;
    let dz = pos1.z - pos2.z;
    let distance_squared = dx * dx + dy * dy + dz * dz;
    let radius_sum = radius1 + radius2;
    
    distance_squared < (radius_sum * radius_sum)
}

/// Verifica colisión de la spaceship con cualquier planeta
pub fn check_spaceship_collisions(
    ship_pos: Vector3,
    ship_radius: f32,
    planets: &[(Vector3, f32)], // (posición, radio)
) -> Option<usize> {
    for (i, (planet_pos, planet_radius)) in planets.iter().enumerate() {
        if check_sphere_collision(ship_pos, ship_radius, *planet_pos, *planet_radius) {
            return Some(i);
        }
    }
    None
}
