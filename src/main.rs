mod intersections;

use image::ImageBuffer;
use intersections::*;
use nalgebra::Vector3;

fn main() {

    let mut img = ImageBuffer::new(2048, 2048);
    let dimensions = img.dimensions();

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        
        let direction = Vector3::new(x as f64 / dimensions.0 as f64 - 0.5, 0.5 - y as f64 / dimensions.1 as f64, 1.0);
        let starting_position = Vector3::new(0.0, 0.0, 0.0);

        let intersection = intersect_sphere(starting_position, direction, Vector3::new(0.0, 0.0, 4.0), 1.0);
        let plane_intersection = intersect_plane(starting_position, direction, Vector3::new(0.0, 0.0, 10.0), Vector3::new(0.0, 0.0, -1.0));

        if let Some(intersection) = intersection {
            let mut color = Vector3::new(255.0, 0.0, 0.0) * intersection.normal.dot(&Vector3::new(1.0, 1.0, -1.0).normalize());
            color.x = (if plane_intersection.is_some() && plane_intersection.unwrap().distance < intersection.distance {
                255.0
            } else {
                color.x
            }) as f64;
            *pixel = image::Rgb([color.x as u8, color.y as u8, color.z as u8]);
        } else {
            let mut color = Vector3::new(0.0, 0.0, 0.0);
            color.x = (if plane_intersection.is_some() {
                255.0
            } else {
                color.x
            }) as f64;
            *pixel = image::Rgb([color.x as u8, color.y as u8, color.z as u8]);
        }

    }

    img.save("test.png").unwrap();

}