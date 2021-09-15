#![allow(unused_imports)]
#![allow(dead_code)]

use std::{collections::HashMap, f64::consts::PI};

use image::{codecs::png::PngEncoder, EncodableLayout, ImageError};
use ndarray::{prelude::*, Zip};

use rayon::prelude::*;

use rand::*;

use derive_builder::Builder;

mod utils;
use utils::*;

mod renderer;
use renderer::*;

mod objects;
use objects::*;

mod materials;
use materials::*;

mod primitive_types;
use primitive_types::*;

use indicatif::ProgressBar;

const SCENE: u32 = 1;

#[cfg(debug_assertions)]
const NORMAL: bool = true;

fn main() {
    // Set the number of threads
    rayon::ThreadPoolBuilder::new()
        .num_threads(16)
        .build_global()
        .unwrap();

    // Image
    let mut canvas = Canvas {
        width: 400,
        height: 225,
        aspect_ratio: 16.0 / 9.0,
        buffer: Array3::zeros((225, 400, 3)),
    };
    let samples_per_pix = 100;
    let depth = 50;

    // World
    let materials = build_materials();
    let objs = build_objects(&materials);
    let scene_objs: Vec<&Object> = objs.iter().collect();

    // Camera
    let camera = Camera::default();

    // Geometry
    let origin: Array1<f64> = Array1::zeros(3);
    let vertical = array![0.0, camera.view_height, 0.0];
    let horizontal = array![camera.view_width, 0.0, 0.0];
    // TODO why negative?
    let top_left_corner =
        &origin - (&horizontal / 2.0) + (&vertical / 2.0) - array![0.0, 0.0, camera.focal_length];

    // Progress Bar
    let pixel_count = (canvas.width * canvas.height) as u64;
    let progress_bar = ProgressBar::new(pixel_count);

    // Render
    Zip::indexed(canvas.buffer.lanes_mut(Axis(2))).par_for_each(|(j, i), mut pixel| {
        let mut accum_color = array![0.0, 0.0, 0.0];
        for _ in 0..samples_per_pix {
            let mut rng = thread_rng();
            let u = (i as f64 + rng.gen::<f64>()) / (canvas.width - 1) as f64;
            let v = (j as f64 + rng.gen::<f64>()) / (canvas.height - 1) as f64;
            let ray = Ray {
                origin: Array1::zeros(3),
                direction: (&top_left_corner + u * &horizontal - v * &vertical - &origin),
            };
            accum_color += &ray.get_color(&scene_objs, depth);
        }
        // TODO allow manual gamma correction
        pixel.assign(&(&accum_color / samples_per_pix as f64).mapv(|x| x.sqrt()));
        progress_bar.inc(1);
    });

    // I/O
    canvas.save().expect("Failed to save file.");
}

fn build_materials() -> HashMap<&'static str, Material> {
    let red_diffuse = Material::Diffuse(
        DiffuseBuilder::default()
            .albedo(array![1.0, 0.0, 0.0])
            .build()
            .unwrap(),
    );
    let blue_diffuse = Material::Diffuse(
        DiffuseBuilder::default()
            .albedo(array![0.0, 0.0, 1.0])
            .build()
            .unwrap(),
    );
    let mut materials = HashMap::new();
    materials.insert("red_diffuse", red_diffuse);
    materials.insert("blue_diffuse", blue_diffuse);
    materials
}

fn build_objects<'a>(materials: &'a HashMap<&'static str, Material>) -> Vec<Object<'a>> {
    let r = (PI / 4.0).cos();
    let sphere_right = Object::Sphere(
        SphereBuilder::default()
            .center(array![r, 0.0, -1.0])
            .radius(r)
            .material(&materials.get("red_diffuse").unwrap())
            .build()
            .unwrap(),
    );
    let sphere_left = Object::Sphere(
        SphereBuilder::default()
            .center(array![-r, 0.0, -1.0])
            .radius(r)
            .material(&materials.get("blue_diffuse").unwrap())
            .build()
            .unwrap(),
    );
    vec![sphere_right, sphere_left]
}