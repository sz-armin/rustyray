use std::collections::HashMap;

use image::{codecs::png::PngEncoder, EncodableLayout, ImageError};
use ndarray::{prelude::*, Zip};

#[allow(unused_imports)]
use rayon::prelude::*;

use rand::distributions::Distribution;
use rand::distributions::Uniform;

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

use nalgebra::*;

#[cfg(debug_assertions)]
const NORMAL: bool = false;

fn main() {
    // Set the number of threads
    rayon::ThreadPoolBuilder::new()
        .num_threads(16)
        .build_global()
        .unwrap();

    // Image
    let mut canvas = Canvas {
        width: 600,
        height: 400,
        aspect_ratio: 3.0 / 2.0,
        buffer: Array3::zeros((400, 600, 3)),
    };
    let samples_per_pix = 100;
    let depth = 10;

    // World
    let materials = build_materials();
    let objs = build_objects(&materials);
    let scene_objs: Vec<&Object> = objs.iter().collect();

    // Camera
    let camera = CameraBuilder::default()
        .aspect_ratio(canvas.aspect_ratio)
        .origin(vector![13.0, 2.0, 3.0])
        .vfov(20.0)
        .focus_dist(10.0)
        .look_at(vector![0.0, 0.0, 0.0])
        .aperture(0.1)
        .build()
        .unwrap()
        .finalize_build();

    // Progress Bar
    let pixel_count = (canvas.width * canvas.height) as u64;
    let progress_bar = ProgressBar::new(pixel_count);

    // Render
    Zip::indexed(canvas.buffer.lanes_mut(Axis(2))).par_for_each(|(j, i), mut pixel| {
        let mut accum_color = vector![0.0, 0.0, 0.0];
        let mut rng = thread_rng();
        for _ in 0..samples_per_pix {
            let u = (i as f64 + rng.gen::<f64>()) / (canvas.width - 1) as f64;
            let v = (j as f64 + rng.gen::<f64>()) / (canvas.height - 1) as f64;
            // TODO move to camera
            let ray = camera.get_ray(u, v);
            accum_color += &ray.get_color(&scene_objs, depth);
        }
        // TODO allow manual gamma correction
        let arr = Array1::from_iter(
            (accum_color / samples_per_pix as f64)
                .map(|x| x.sqrt())
                .iter()
                .cloned(),
        );
        pixel.assign(&arr);
        progress_bar.inc(1);
    });

    // I/O
    canvas.save().expect("Failed to save file.");
}

fn build_materials() -> HashMap<String, Material> {
    let mut materials = HashMap::new();

    let mut rng = thread_rng();
    for i in 0..484 {
        let choose_mat = rng.gen::<f64>();
        if choose_mat < 0.8 {
            let albedo =
                Vector3::from_distribution(&Uniform::new(0.0, 1.0), &mut rng).map(|x| x * x);
            let material =
                Material::Diffuse(DiffuseBuilder::default().albedo(albedo).build().unwrap());
            materials.insert(format!("{}", i), material);
        } else if choose_mat < 0.95 {
            let albedo = Vector3::from_distribution(&Uniform::new(0.5, 1.0), &mut rng);
            let fuzz = rng.gen::<f64>() / 2.0;
            let material = Material::Metal(
                MetalBuilder::default()
                    .albedo(albedo)
                    .fuzziness(fuzz)
                    .build()
                    .unwrap(),
            );
            materials.insert(format!("{}", i), material);
        } else {
            let material = Material::Glass(GlassBuilder::default().ir(1.5).build().unwrap());
            materials.insert(format!("{}", i), material);
        }
    }

    let material_ground = Material::Diffuse(
        DiffuseBuilder::default()
            .albedo(vector![0.5, 0.5, 0.5])
            .build()
            .unwrap(),
    );
    let sphere2 = Material::Diffuse(
        DiffuseBuilder::default()
            .albedo(vector![0.4, 0.2, 0.1])
            .build()
            .unwrap(),
    );
    let sphere1 = Material::Glass(GlassBuilder::default().ir(1.5).build().unwrap());
    let sphere3 = Material::Metal(
        MetalBuilder::default()
            .albedo(vector![0.7, 0.6, 0.5])
            .build()
            .unwrap(),
    );

    materials.insert("material_ground".to_string(), material_ground);
    materials.insert("sphere1".to_string(), sphere1);
    materials.insert("sphere2".to_string(), sphere2);
    materials.insert("sphere3".to_string(), sphere3);
    materials
}

fn build_objects(materials: &HashMap<String, Material>) -> Vec<Object> {
    let mut rng = thread_rng();
    let mut objects = vec![];
    let mut counter = 0;

    for a in -11..11 {
        for b in -11..11 {
            // let choose_mat = rng.gen::<f64>();
            let center = vector![
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>()
            ];
            let temp = center - vector![4.0, 0.2, 0.0];
            if temp.dot(&temp).sqrt() > 0.9 {
                let sphere = Object::Sphere(
                    SphereBuilder::default()
                        .center(center)
                        .radius(0.2)
                        .material(materials.get(&format!("{}", counter)).unwrap())
                        .build()
                        .unwrap(),
                );
                objects.push(sphere);
                counter += 1;
            }
        }
    }

    let sphere_ground = Object::Sphere(
        SphereBuilder::default()
            .center(vector![0.0, -1000.0, 0.0])
            .radius(1000.0)
            .material(materials.get("material_ground").unwrap())
            .build()
            .unwrap(),
    );
    let sphere1 = Object::Sphere(
        SphereBuilder::default()
            .center(vector![0.0, 1.0, 0.0])
            .radius(1.0)
            .material(materials.get("sphere1").unwrap())
            .build()
            .unwrap(),
    );
    let sphere2 = Object::Sphere(
        SphereBuilder::default()
            .center(vector![-4.0, 1.0, 0.0])
            .radius(1.0)
            .material(materials.get("sphere2").unwrap())
            .build()
            .unwrap(),
    );
    let sphere3 = Object::Sphere(
        SphereBuilder::default()
            .center(vector![4.0, 1.0, 0.0])
            .radius(1.0)
            .material(materials.get("sphere3").unwrap())
            .build()
            .unwrap(),
    );
    objects.push(sphere_ground);
    objects.push(sphere1);
    objects.push(sphere2);
    objects.push(sphere3);
    objects
}
