use cgmath::{Vector3, InnerSpace};
use rand::{thread_rng, Rng};

use crate::intersections::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectType {

    Sphere {
        radius: f64,
    },
    Plane {
        normal: Vector3<f64>,
    },

}

#[derive(Debug, Clone, PartialEq)]
pub struct Material {

    pub color : Vector3<f64>,
    pub roughness: f64,
    pub refractivity: f64,

}


pub fn generate_random_vector() -> Vector3<f64> {

    Vector3::new(thread_rng().gen_range(-1.0..1.0), thread_rng().gen_range(-1.0..1.0), thread_rng().gen_range(-1.0..1.0)).normalize()

}

impl Material {

    pub fn new(color: Vector3<f64>, roughness: f64, refractivity: f64) -> Material {

        Material { 
            
            color,
            roughness,
            refractivity,

        }

    }

    pub fn generate_random_vector(&self, incoming_direction: Vector3<f64>, normal: Vector3<f64>) -> Vector3<f64> {

        let normal = normal.normalize();
        let incoming_direction = incoming_direction.normalize();
        let mut diffuse_direction = generate_random_vector();
        diffuse_direction = diffuse_direction * normal.dot(diffuse_direction).signum();
        let mut specular_direction = incoming_direction.normalize() - normal * 2.0 * normal.dot(incoming_direction.normalize());
        specular_direction = specular_direction.normalize();

        diffuse_direction * self.roughness + specular_direction * (1.0 - self.roughness)
        

    }


}

#[derive(Debug, Clone, PartialEq)]
pub struct IntersectableObject { 

    pub object_type: ObjectType,
    pub material: Material,
    pub position: Vector3<f64>,

}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectIntersection<'a> {

    pub object: &'a IntersectableObject,
    pub intersection: Intersection,

}

pub fn intersect_object(starting_position: Vector3<f64>, direction: Vector3<f64>, object: &IntersectableObject) -> Option<ObjectIntersection> { // This could be done with traits, but this means things would have to be boxed, plus there are only so many primitives, and as long as we have the triangle we can basically make anything.

    let position = object.position;

    match object.object_type {
        ObjectType::Sphere { radius } => {
            let intersection = intersect_sphere(starting_position, direction, position, radius);
            if let Some(intersection) = intersection {
                Some(ObjectIntersection {
                    object: &object,
                    intersection,
                })
            } else {
                None
            }
        },
        ObjectType::Plane { normal } => {
            let intersection = intersect_plane(starting_position, direction, position, normal);
            if let Some(intersection) = intersection {
                Some(ObjectIntersection {
                    object: &object,
                    intersection,
                })
            } else {
                None
            }
        },
    }

}

pub fn intersect_objects(starting_position: Vector3<f64>, direction: Vector3<f64>, objects: &Vec<IntersectableObject>) -> Option<ObjectIntersection> {

    let intersection = objects.iter().map(|object| {
        intersect_object(starting_position, direction, object)
    }).filter(|intersection| {
        intersection.is_some()
    }).min_by(|a, b| {
        a.as_ref().unwrap().intersection.distance.partial_cmp(&b.as_ref().unwrap().intersection.distance).unwrap_or(std::cmp::Ordering::Greater)
    });

    if intersection.is_some() {
        intersection.unwrap()
    } else {
        None
    }

}

pub fn get_random_point_on_surface(object: &IntersectableObject) -> Vector3<f64> {

    let position = object.position;

    match object.object_type {
        ObjectType::Sphere { radius } => {

            position + Vector3::new(thread_rng().gen_range(-1.0..1.0), thread_rng().gen_range(-1.0..1.0), thread_rng().gen_range(-1.0..1.0)).normalize() * radius

        },
        ObjectType::Plane { normal } => {

            let new_position = Vector3::new(thread_rng().gen_range(-1.0..1.0), thread_rng().gen_range(-1.0..1.0), thread_rng().gen_range(-1.0..1.0)).normalize() * 10.0;
            position + new_position - normal * (new_position.dot(normal))

        },
    }
}