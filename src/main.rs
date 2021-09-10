#![allow(unused_imports)]
#![allow(dead_code)]

use path_tracer;
use std::ops::{self, Add, Mul};

use image::{codecs::png::PngEncoder, EncodableLayout, ImageError, Pixel};
use ndarray::{prelude::*, DataOwned, OwnedRepr, RawData, Zip};

use rayon::prelude::*;

fn main() {
    // Image
    let mut canvas = Canvas {
        width: 400,
        height: 225,
        aspect_ratio: 16.0 / 9.0,
        buffer: Array3::zeros((225, 400, 3)),
    };

    // Camera
    let viewport = ViewPort {
        height: 2.0,
        width: 2.0 * 16.0 / 9.0,
    };
    let camera = Camera {
        viewport,
        focal_length: 1.0,
    };

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
        pixel.assign(&ray.get_color());
    });

    // I/O
    canvas.save().expect("Failed to save file.");
}

#[derive(Debug)]
struct Ray {
    origin: Array1<f32>,
    direction: Array1<f32>,
}

impl Ray {
    fn get_color(&self) -> Array1<f32> {
        let sphere = Sphere {
            center: array![0.0, 0.0, -1.0],
            radius: 0.5,
        };
        match self.hit_sphere(&sphere) {
            // TODO unneeded?
            Some(t) if t > 0.0 => {
                let normal = (self.at(t) - sphere.center).unit();
                return 0.5 * array![normal[0] + 1.0, normal[1] + 1.0, normal[2] + 1.0];
            }
            None => (),
            _ => (),
        }

        let t = 0.5 * (self.direction.unit()[1] + 1.0);
        // Colors
        (1.0 - t) * array![1.0, 1.0, 1.0] + t * array![0.5, 0.7, 1.0]
    }

    fn hit_sphere(&self, sphere: &Sphere) -> Option<f32> {
        let oc = &self.origin - &sphere.center;
        let a = self.direction.dot(&self.direction);
        let b = 2.0 * oc.dot(&self.direction);
        let c = oc.dot(&oc) - sphere.radius.powi(2);

        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        } else {
            return Some((-b - discriminant.sqrt()) / (2.0 * a));
        }
    }

    fn at(&self, t: f32) -> Array1<f32> {
        &self.origin + t * &self.direction
    }
}
struct ViewPort {
    height: f32,
    width: f32,
}

#[derive(Debug, Clone)]
struct Point(u32);
impl ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            0: self.0 + other.0,
        }
    }
}
struct Direction;
struct Camera {
    viewport: ViewPort,
    focal_length: f32,
}
struct Canvas {
    width: u32,
    height: u32,
    aspect_ratio: f32,
    buffer: Array3<f32>,
}

impl Canvas {
    fn save(&self) -> Result<(), ImageError> {
        let tmapped_raw = self
            .buffer
            .as_standard_layout()
            .mapv(|x| (x * 255.0) as u8)
            .into_raw_vec();
        let file = std::fs::File::create("/home/qtqbpo/a.png").unwrap();
        let encoder = PngEncoder::new(file);
        encoder.encode(
            tmapped_raw.as_bytes(),
            self.width,
            self.height,
            image::ColorType::Rgb8,
        )?;
        Ok(())
    }
}

struct Sphere {
    center: Array1<f32>,
    radius: f32,
}

trait Unit {
    fn unit(&self) -> Array1<f32>;
}

impl Unit for Array1<f32> {
    fn unit(&self) -> Array1<f32> {
        self.clone() / (self * self).sum().sqrt()
    }
}
