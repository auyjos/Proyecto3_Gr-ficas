use crate::fragment::Fragment;
use crate::vertex::Vertex;
use raylib::prelude::Vector3;

pub fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    // Get screen coordinates
    let p1 = v1.transformed_position;
    let p2 = v2.transformed_position;
    let p3 = v3.transformed_position;

    // Find bounding box
    let min_x = (p1.x.min(p2.x).min(p3.x)).floor() as i32;
    let max_x = (p1.x.max(p2.x).max(p3.x)).ceil() as i32;
    let min_y = (p1.y.min(p2.y).min(p3.y)).floor() as i32;
    let max_y = (p1.y.max(p2.y).max(p3.y)).ceil() as i32;

    // Helper function to compute barycentric coordinates
    fn sign(p1: Vector3, p2: Vector3, p3: Vector3) -> f32 {
        (p1.x - p3.x) * (p2.y - p3.y) - (p2.x - p3.x) * (p1.y - p3.y)
    }

    // Iterate through all pixels in bounding box
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let pt = Vector3::new(x as f32, y as f32, 0.0);

            let d1 = sign(pt, v1.transformed_position, v2.transformed_position);
            let d2 = sign(pt, v2.transformed_position, v3.transformed_position);
            let d3 = sign(pt, v3.transformed_position, v1.transformed_position);

            let has_neg = (d1 < 0.0) || (d2 < 0.0) || (d3 < 0.0);
            let has_pos = (d1 > 0.0) || (d2 > 0.0) || (d3 > 0.0);

            if !(has_neg && has_pos) {
                // Point is inside triangle - interpolate depth AND COLOR
                let total = (d1.abs() + d2.abs() + d3.abs()).max(0.0001);
                let w1 = d1.abs() / total;
                let w2 = d2.abs() / total;
                let w3 = d3.abs() / total;

                let z = p1.z * w1 + p2.z * w2 + p3.z * w3;
                
                // INTERPOLATE THE VERTEX COLORS (this is the texture color!)
                let color = Vector3::new(
                    v1.color.x * w1 + v2.color.x * w2 + v3.color.x * w3,
                    v1.color.y * w1 + v2.color.y * w2 + v3.color.y * w3,
                    v1.color.z * w1 + v2.color.z * w2 + v3.color.z * w3,
                );

                fragments.push(Fragment::new(x as f32, y as f32, color, z));
            }
        }
    }

    fragments
}
