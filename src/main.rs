#![allow(unused_imports)]
#![allow(dead_code)]

use std::ops;

use image::{codecs::png::PngEncoder, EncodableLayout, ImageError};
use ndarray::{prelude::*, Zip};

use rayon::prelude::*;

pub mod renderer;
pub use renderer::*;

pub mod objects;
pub use objects::*;

fn main() {
    // Image
    let mut canvas = Canvas {
        width: 400,
        height: 225,
        aspect_ratio: 16.0 / 9.0,
        buffer: Array3::zeros((225, 400, 3)),
    };

    // World
    // point3(0,-100.5,-1), 100))
    let sphere1 = Object::Sphere(Sphere {
        center: array![0.0, 0.0, -1.0],
        radius: 0.5,
    });
    let sphere2 = Object::Sphere(Sphere {
        center: array![0.0, -100.5, -1.0],
        radius: 100.0,
    });
    let scene_objs: Vec<&Object> = vec![&sphere1, &sphere2];

    // Camera
    let camera = Camera::new();

    // Geometry
    let origin: Array1<f32> = Array1::zeros(3);
    let vertical = array![0.0, camera.viewport.height, 0.0];
    let horizontal = array![camera.viewport.width, 0.0, 0.0];
    // TODO why negative?
    let top_left_corner =
        &origin - (&horizontal / 2.0) + (&vertical / 2.0) - array![0.0, 0.0, camera.focal_length];

    // Render
    Zip::indexed(canvas.buffer.lanes_mut(Axis(2))).par_for_each(|(j, i), mut pixel| {
        let u = i as f32 / (canvas.width - 1) as f32;
        let v = j as f32 / (canvas.height - 1) as f32;
        let ray = Ray {
            origin: Array1::zeros(3),
            direction: &top_left_corner + u * &horizontal - v * &vertical - &origin,
        };
        pixel.assign(&ray.get_color(&scene_objs));
    });

    // I/O
    canvas.save().expect("Failed to save file.");
}

// #[derive(Debug, Clone)]
// struct Point(u32);
// impl ops::Add for Point {
//     type Output = Self;

//     fn add(self, other: Self) -> Self {
//         Self {
//             0: self.0 + other.0,
//         }
//     }
// }
// struct Direction;
