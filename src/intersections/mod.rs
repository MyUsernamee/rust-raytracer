use nalgebra::Vector3;


pub struct Intersection {

    pub starting_position: Vector3<f64>,
    pub starting_direction: Vector3<f64>,
    pub intersection_position: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub distance: f64,

}

pub fn intersect_sphere(starting_position: Vector3<f64>, starting_direction: Vector3<f64>, sphere_position: Vector3<f64>, sphere_radius: f64) -> Option<Intersection> {

    let direction = starting_direction.normalize();
    let starting_offset = sphere_position - starting_position;
    let a = starting_offset.magnitude() / direction.dot(&starting_offset.normalize()); // Multiplier to project the direction to the position of the sphere
    let b = a * a - starting_offset.magnitude_squared(); // Distance between the projection and the sphere

    if b > sphere_radius * sphere_radius || b < 0.0 || a < 0.0 { // If the distance from the projected point towards the sphere is farther than then raidus of the sphere, then we didn't hit
        return None;
    }

    let c = ((sphere_radius * sphere_radius) - b).sqrt(); // We use the distance as the sphere as one leg of a triangle and the hypotenuse of the triangle is the radius of the sphere.
    let t = a - c; // Move the projection back to be sitting on the surface of the sphere.

    let intersection_position = starting_position + direction * (t);
    let normal = (intersection_position - sphere_position).normalize();

    Some(Intersection {
        starting_position: starting_position,
        starting_direction: starting_direction,
        intersection_position: intersection_position,
        normal: normal,
        distance: t,
    })

}

pub fn intersect_plane(starting_position: Vector3<f64>, starting_direction: Vector3<f64>, plane_position: Vector3<f64>, plane_normal: Vector3<f64>) -> Option<Intersection> {

    let direction = starting_direction.normalize();
    let dot = plane_normal.dot(&direction);

    if dot >= 0.0 {
        return None;
    }

    let distance = plane_normal.dot(&(plane_position - starting_position)) / dot;
    let intersection_position = starting_position + direction * distance;
    let normal = plane_normal;

    Some(Intersection {
        starting_position: starting_position,
        starting_direction: starting_direction,
        intersection_position: intersection_position,
        normal: normal,
        distance: distance,
    })
    

}