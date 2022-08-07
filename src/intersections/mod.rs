use cgmath::{Vector3, InnerSpace};

#[derive(Debug, Clone, PartialEq)]
pub struct Intersection {

    pub starting_position: Vector3<f64>,
    pub starting_direction: Vector3<f64>,
    pub intersection_position: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub distance: f64,
    pub exit_distance: f64,

}

pub fn intersect_sphere(starting_position: Vector3<f64>, starting_direction: Vector3<f64>, sphere_position: Vector3<f64>, sphere_radius: f64) -> Option<Intersection> {

    let direction = starting_direction.normalize();
    let starting_offset = sphere_position - starting_position;
    let a = direction.dot(starting_offset); // Multiplier to project the direction to the position of the sphere
    let b = starting_offset.magnitude2() - a * a; // Distance between the projection and the sphere

    if b >= sphere_radius * sphere_radius { // If the distance from the projected point towards the sphere is farther than then raidus of the sphere, then we didn't hit
        return None;
    }

    let c = ((sphere_radius * sphere_radius) - b).sqrt(); // We use the distance as the sphere as one leg of a triangle and the hypotenuse of the triangle is the radius of the sphere.
    let mut t = a - c; // The distance from the starting position to the intersection point
    let mut t2 = a + c; // The distance from the starting position to the exit point
    let mut inside_sphere = false;
    
    if t > t2 { // If t2 is closer, swap them
        let t_temp = t.clone();
        t = t2;
        t2 = t_temp;
    }

    if t < 0.0 { // Dang rounding errors...
        return None;
    }

    let intersection_position = starting_position + direction * (t);
    let normal = (intersection_position - sphere_position).normalize() * (if inside_sphere { -1.0 } else { 1.0 });
    let intersection_position = intersection_position + normal * 0.001; // Move the intersection point a little bit away from the sphere to avoid self-intersections.

    Some(Intersection {
        starting_position: starting_position,
        starting_direction: starting_direction,
        intersection_position: intersection_position,
        normal: normal,
        distance: t,
        exit_distance: t2,
    })

}

pub fn intersect_plane(starting_position: Vector3<f64>, starting_direction: Vector3<f64>, plane_position: Vector3<f64>, plane_normal: Vector3<f64>) -> Option<Intersection> {

    let direction = starting_direction.normalize();
    let dot = plane_normal.dot(direction);

    if dot >= 0.0 {
        return None;
    }

    let distance = plane_normal.dot((plane_position - starting_position)) / dot;
    let intersection_position = starting_position + direction * distance;
    let normal = plane_normal;

    Some(Intersection {
        starting_position: starting_position,
        starting_direction: starting_direction,
        intersection_position: intersection_position,
        normal: normal,
        distance: distance,
        exit_distance: distance, // A plane is only one sided.
    })
    

}