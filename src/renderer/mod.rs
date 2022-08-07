use cgmath::{Vector3, Matrix4, InnerSpace, Matrix, ElementWise, Matrix3, SquareMatrix, MetricSpace};
use image::{ImageBuffer, Rgb, Pixel};
use rand::{thread_rng, Rng};

use crate::{object::{IntersectableObject, intersect_objects, get_random_point_on_surface, intersect_object, ObjectIntersection}, intersections::Intersection};

pub struct Camera {

    pub position: Vector3<f64>,
    pub direction: Vector3<f64>,
    pub fov: f64,
    pub aspect_ratio: f64,
    pub resolution: (u32, u32),

}

pub fn render(camera: &Camera, objects: &Vec<IntersectableObject>, samples: usize, bounces: usize) -> ImageBuffer<Rgb<u8>, Vec<u8>> {

    let mut image = ImageBuffer::new(camera.resolution.0, camera.resolution.1);
    let mut sample_buffer: ImageBuffer<Rgb<f64>, Vec<f64>> = ImageBuffer::new(camera.resolution.0, camera.resolution.1);

    let camera_right = camera.direction.cross(Vector3::new(0.0, 1.0, 0.0)).normalize();
    let camera_up = camera_right.cross(camera.direction).normalize();

    let camera_matrix = Matrix4::new(
        camera_right.x, camera_up.x, camera.direction.x, 0.0,
        camera_right.y, camera_up.y, camera.direction.y, 0.0,
        camera_right.z, camera_up.z, camera.direction.z, 0.0,
        camera.position.x, camera.position.y, camera.position.z, 1.0,
    );

    let lights: Vec<&IntersectableObject> = objects.iter().filter(|object| {
        object.material.color.x > 1.0 || object.material.color.y > 1.0 || object.material.color.z > 1.0
    }).collect();

    for sample in 0..samples {

        for (x, y, pixel) in sample_buffer.enumerate_pixels_mut() {

            let mut direction = Vector3::new(x as f64 / camera.resolution.0 as f64 - 0.5, 0.5 - y as f64 / camera.resolution.1 as f64, camera.fov).normalize();
            let mut starting_position = Vector3::new(0.0, 0.0, 0.0);

            direction = Matrix3::from_cols(camera_matrix.x.truncate(), camera_matrix.y.truncate(), camera_matrix.z.truncate()).transpose() * direction;
            starting_position = (camera_matrix * starting_position.extend(1.0)).truncate();
            
            let color = trace(starting_position, direction, objects, &lights, bounces);

            *pixel = Rgb([pixel.0[0] + color.x / samples as f64, pixel.0[1] + color.y / samples as f64, pixel.0[2] + color.z / samples as f64]);

        }

    }

    for (x, y, pixel) in image.enumerate_pixels_mut() {

        let final_color = sample_buffer.get_pixel(x, y).map(|p| (p / (p + 1.0)) * 255.0);
        *pixel = Rgb([final_color.0[0] as u8, final_color.0[1] as u8, final_color.0[2] as u8]);

    }

    image



}

pub fn trace(origin: Vector3<f64>, direction: Vector3<f64>, objects: &Vec<IntersectableObject>, lights: &Vec<&IntersectableObject>, bounces: usize) -> Vector3<f64> {

    let mut color = Vector3::new(0.0, 0.0, 0.0);
    let mut albedo = Vector3::new(1.0, 1.0, 1.0);
    let mut intersection = intersect_objects(origin, direction, objects);
    
    let mut total_bounces = bounces;

    for bounce in 0..bounces {
        if let Some(hit_intersection) = intersection {

            // When we hit the light, we don't want to continue bouncing
            if (hit_intersection.object.material.color.x > 1.0 || hit_intersection.object.material.color.y > 1.0 || hit_intersection.object.material.color.z > 1.0) {
                color += albedo.mul_element_wise(hit_intersection.object.material.color);
                break;
            }

            
            total_bounces += bounce + 1;
            
            // Calculate light contribution
            let light_contribution = {
                let mut contribution = Vector3::new(0.0, 0.0, 0.0);
                if hit_intersection.object.material.roughness < 1.0 {
                    // Set light contribution to black
                    contribution = Vector3::new(0.0, 0.0, 0.0);
                }
                else {
                    for light in lights {

                        let sample_position = get_random_point_on_surface(&light);
                        let sample_direction = sample_position - hit_intersection.intersection.intersection_position;
                        let sample_direction = sample_direction.normalize();

                        let light_intersection = intersect_objects(hit_intersection.intersection.intersection_position, sample_direction, &objects);
                        if let Some(light_intersection) = light_intersection {
                            if &light_intersection.object != light { continue; }
                            let light_distance = light_intersection.intersection.distance;
                            let light_color = light_intersection.object.material.color;
                            let light_contribution = contribution + light_color / (light_distance * light_distance) * (hit_intersection.intersection.normal.dot(sample_direction)).powf(1.0 / hit_intersection.object.material.roughness);
                            contribution += light_contribution;
                        }

                    }
                }
                contribution
            };

            albedo = albedo.mul_element_wise(hit_intersection.object.material.color); // Multiply albedo
            color += albedo.mul_element_wise(light_contribution); // Accumlate path
            
            // Select new direction and recast ray.
            let random_vector = hit_intersection.object.material.generate_random_vector(hit_intersection.intersection.starting_direction, hit_intersection.intersection.normal);
            intersection = intersect_objects(hit_intersection.intersection.intersection_position, random_vector, objects);

        } else {
            break;
        }
    }
    color / bounces as f64

}

#[derive(Clone, PartialEq)]
struct BounceData<'a> {

    outgoing_light: Vector3<f64>,
    intersection: ObjectIntersection<'a>,

}

pub fn trace_bdpt(origin: Vector3<f64>, direction: Vector3<f64>, objects: &Vec<IntersectableObject>, light: &IntersectableObject, bounces: usize) -> Vector3<f64> {

    let mut light_bounce_points : Vec<BounceData> = Vec::new();
    light_bounce_points.push(
        BounceData {
            outgoing_light: light.material.color,
            intersection: ObjectIntersection {
                object: light,
                intersection: Intersection { 
                    starting_position: Vector3::new(0.0, 0.0, 0.0),
                    intersection_position: get_random_point_on_surface(light),
                    normal: 
                    distance: 0.0,
                    exit_distance: 0.0,
                    starting_direction: Vector3::new(0.0, 0.0, 0.0) 
                }
            },
        }
    );

    for i in 0..bounces {
        let last_bounce = light_bounce_points.last().unwrap();
        intersect_objects(last_bounce.intersection.interseciont.starting_position, , objects)
    }
    
}