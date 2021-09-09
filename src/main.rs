#![allow(unused_imports)]
#![allow(dead_code)]

use path_tracer;
use std::ops;

use image::{DynamicImage, ImageBuffer, ImageError, ImageFormat, ImageResult, Rgb, RgbImage};
use ndarray::{prelude::*, Slice};

use itertools::{Itertools, Zip};
use std::slice::Chunks;

use rayon::prelude::*;

use ndarray_linalg::{normalize, Norm, NormalizeAxis};

fn main() {
    // Image
    let mut canvas = Canvas {
        width: 400,
        height: 225,
        aspect_ratio: 16.0 / 9.0,
        buffer: Array3::zeros((400, 225, 3)),
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

    let origin: Array1<f32> = Array1::zeros(3);
    let vertical = array![0.0, camera.viewport.height, 0.0];
    let horizontal = array![camera.viewport.width, 0.0, 0.0];
    let lower_left_corner = origin.clone()
        - (&horizontal / 2.0)
        - (&vertical / 2.0)
        - array![0.0, 0.0, camera.focal_length];

    dbg!(&lower_left_corner);

    // canvas.buffer.exact_chunks_mut((1,1,3)).map(|x| x.fill(255.0));
    canvas
        .buffer
        .lanes_mut(Axis(2))
        .into_iter()
        .enumerate()
        .for_each(|(ind, mut pixel)| {
            let i = ind % canvas.width as usize;
            let j = ind / canvas.width as usize;
            let u = i as f32 / (canvas.width - 1) as f32;
            let v = j as f32 / (canvas.height - 1) as f32;
            let ray = Ray {
                origin: Array1::zeros(3),
                direction: lower_left_corner.clone() + u * &horizontal + v * &vertical - &origin,
            };
            pixel.assign(&ray.get_color());
        });
    canvas.buffer[[0,50, 0]] = 0.0;
    canvas.buffer[[0,50, 1]] = 0.0;
    canvas.buffer[[0,50, 2]] = 0.0;
    canvas.save().expect("Failed to save file.");
}

fn color_ray() {
    unimplemented!()
}

struct Ray {
    origin: Array1<f32>,
    direction: Array1<f32>,
}

impl Ray {
    fn get_color(&self) -> Array1<f32> {
        // auto t = 0.5*(unit_direction.y() + 1.0);
        // return (1.0-t)*color(1.0, 1.0, 1.0) + t*color(0.5, 0.7, 1.0);
        let normalized_dir = self.direction.clone() / self.direction.norm_l2();
        let t = 0.5 * (normalized_dir[1] + 1.0);
        // Colors
        (1.0 - t) * array![1.0, 1.0, 1.0] + t * array![0.5, 0.7, 1.0]
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
        let image = RgbImage::from_raw(self.width as u32, self.height as u32, tmapped_raw)
            .expect("container should have the right size for the image dimensions");
        image.save_with_format("/home/qtqbpo/a.png", ImageFormat::Png)?;
        Ok(())
    }
}
