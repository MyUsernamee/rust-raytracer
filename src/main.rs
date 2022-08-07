mod intersections;
mod object;
mod renderer;

use std::{fs::File, io::Write, f64::consts::TAU, env};

use cgmath::{Matrix4, Vector3, Point3, InnerSpace, Matrix3, Matrix, Transform};
use image::{ImageBuffer, GenericImageView, codecs::{png::FilterType, gif::{GifEncoder, Repeat}}, Rgb, RgbaImage, Frame, DynamicImage, Delay, Rgba};
use intersections::*;
use object::{IntersectableObject, ObjectType, intersect_objects, Material};
use rayon::iter::ParallelBridge;
use renderer::{Camera, render};

const frame_count: usize = 10;

fn main() {

    env::set_var("RUST_BACKTRACE", "1");

    let mut camera_position = Vector3::new(0.0, 0.0, -2.5);
    let mut camera_direction = Vector3::new(0.0, 0.0, 1.0).normalize();

    let mut objects: Vec<IntersectableObject> = Vec::new();
    objects.push(IntersectableObject {
        position: Vector3::new(0.0, 0.0, 0.0),
        object_type: ObjectType::Sphere {
            radius: 1.0,
        },
        material: Material {
            color: Vector3::new(1.0, 0.0, 0.0),
            roughness: 1.0,
            refractivity: 0.0,
        },
    });
    objects.push(IntersectableObject {
        position: Vector3::new(0.0, 100.0 + 1000000.0, 0.0),
        object_type: ObjectType::Sphere {
            radius: 1000000.0,
        },
        material: Material {
            color: Vector3::new(1.0, 1.0, 1.0) * 10.0,
            roughness: 1.0,
            refractivity: 0.0,
        },
    });
    objects.push(IntersectableObject {
        position: Vector3::new(0.0, -1.0, 0.0),
        object_type: ObjectType::Plane {
            normal: Vector3::new(0.0, 1.0, 0.0),
        },
        material: Material {
            color: Vector3::new(1.0, 1.0, 1.0),
            roughness: 1.0,
            refractivity: 0.0,
        },
    });
    let mut frames : Vec<Frame> = Vec::new();

    for image in 0..frame_count {

        let img = render(&Camera {
            position: camera_position,
            direction: camera_direction,
            fov: 0.5,
            aspect_ratio: 1.0,
            resolution: (256, 256),
            
        }, &objects, 4, 8);

        // Change it to a RgbaImage.
        let mut img = DynamicImage::from(img);
        let mut img = img.to_rgba8();

        frames.push(Frame::from_parts(img, 0, 0, Delay::from_numer_denom_ms(1000, frame_count as u32)));

        println!("Rendered frame {}", image);

    }

    let mut file = File::create("animation.gif").unwrap();
    let mut animation = GifEncoder::new(file);
    animation.set_repeat(Repeat::Infinite);
    animation.encode_frames(frames);


}