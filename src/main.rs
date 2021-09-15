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
    let camera = CameraBuilder::default()
        .origin(array![-2.0, 2.0, 1.0])
        .vfov(20.0)
        .build()
        .unwrap()
        .finalize_build();

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
            // TODO move to camera
            let ray = Ray {
                origin: camera.origin.clone(),
                direction: (&camera.top_left_corner + u * &camera.horizontal
                    - v * &camera.vertical
                    - &camera.origin),
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
    let material_ground = Material::Diffuse(
        DiffuseBuilder::default()
            .albedo(array![0.8, 0.8, 0.0])
            .build()
            .unwrap(),
    );
    let material_center = Material::Diffuse(
        DiffuseBuilder::default()
            .albedo(array![0.1, 0.2, 0.5])
            .build()
            .unwrap(),
    );
    let material_left = Material::Glass(GlassBuilder::default().ir(1.5).build().unwrap());
    let material_right = Material::Metal(
        MetalBuilder::default()
            .albedo(array![0.8, 0.6, 0.2])
            .build()
            .unwrap(),
    );
    let mut materials = HashMap::new();
    materials.insert("material_ground", material_ground);
    materials.insert("material_center", material_center);
    materials.insert("material_left", material_left);
    materials.insert("material_right", material_right);
    materials
}

fn build_objects<'a>(materials: &'a HashMap<&'static str, Material>) -> Vec<Object<'a>> {
    let sphere_ground = Object::Sphere(
        SphereBuilder::default()
            .center(array![0.0, -100.5, -1.0])
            .radius(100.0)
            .material(&materials.get("material_ground").unwrap())
            .build()
            .unwrap(),
    );
    let sphere_center = Object::Sphere(
        SphereBuilder::default()
            .center(array![0.0, 0.0, -1.0])
            .radius(0.5)
            .material(&materials.get("material_center").unwrap())
            .build()
            .unwrap(),
    );
    let sphere_left = Object::Sphere(
        SphereBuilder::default()
            .center(array![-1.0, 0.0, -1.0])
            .radius(0.5)
            .material(&materials.get("material_left").unwrap())
            .build()
            .unwrap(),
    );
    let sphere_inside = Object::Sphere(
        SphereBuilder::default()
            .center(array![-1.0, 0.0, -1.0])
            .radius(-0.45)
            .material(&materials.get("material_left").unwrap())
            .build()
            .unwrap(),
    );
    let sphere_right = Object::Sphere(
        SphereBuilder::default()
            .center(array![1.0, 0.0, -1.0])
            .radius(0.5)
            .material(&materials.get("material_right").unwrap())
            .build()
            .unwrap(),
    );
    vec![
        sphere_ground,
        sphere_center,
        sphere_left,
        sphere_inside,
        sphere_right,
    ]
}
