#![allow(unused_imports)]
#![allow(dead_code)]

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
    let material_ground = Material::Diffuse(Diffuse {
        albedo: array![0.8, 0.8, 0.0],
    });
    let material_center = Material::Diffuse(Diffuse {
        albedo: array![0.7, 0.3, 0.3],
    });
    let material_left = Material::Metal(Metal {
        albedo: array![0.8, 0.8, 0.8],
        fuzziness: 0.3,
    });
    let material_right = Material::Metal(Metal {
        albedo: array![0.8, 0.6, 0.2],
        fuzziness: 1.0,
    });

    let sphere1 = Object::Sphere(Sphere {
        center: array![0.0, -100.5, -1.0],
        radius: 100.0,
        material: &material_ground,
    });
    let sphere2 = Object::Sphere(Sphere {
        center: array![0.0, 0.0, -1.0],
        radius: 0.5,
        material: &material_center,
    });
    let sphere3 = Object::Sphere(Sphere {
        center: array![-1.0, 0.0, -1.0],
        radius: 0.5,
        material: &material_left,
    });
    let sphere4 = Object::Sphere(Sphere {
        center: array![1.0, 0.0, -1.0],
        radius: 0.5,
        material: &material_right,
    });

    let scene_objs = vec![&sphere1, &sphere2, &sphere3, &sphere4];

    // Camera
    let camera = Camera::default();

    // Geometry
    let origin: Array1<f64> = Array1::zeros(3);
    let vertical = array![0.0, camera.viewport.height, 0.0];
    let horizontal = array![camera.viewport.width, 0.0, 0.0];
    // TODO why negative?
    let top_left_corner = &origin - (&horizontal / 2.0) + (&vertical / 2.0)
        - array![0.0, 0.0, camera.viewport.focal_length];

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
                direction: &top_left_corner + u * &horizontal - v * &vertical - &origin,
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

    // let diffuse = Material::Diffuse(Diffuse {
    //     albedo: array![0.5, 0.5, 0.5] / 0.5,
    // });

    // let sphere1 = Object::Sphere(Sphere {
    //     center: array![0.0, 0.0, -1.0],
    //     radius: 0.5,
    //     material: &diffuse[0],
    // });
    // let sphere2 = Object::Sphere(Sphere {
    //     center: array![0.0, -100.5, -1.0],
    //     radius: 100.0,
    //     material: &diffuse[0],
    // });

    // let scene_objs = vec![&sphere1, &sphere2];


// Build a hashmap of materials, build a vec of objects, take ref


// 2 
// let material_ground = Material::Diffuse(Diffuse {
//     albedo: array![0.8, 0.8, 0.0],
// });
// let material_center = Material::Diffuse(Diffuse {
//     albedo: array![0.7, 0.3, 0.3],
// });
// let material_left = Material::Metal(Metal {
//     albedo: array![0.8, 0.8, 0.8],
// });
// let material_right = Material::Metal(Metal {
//     albedo: array![0.8, 0.6, 0.2],
// });

// let sphere1 = Object::Sphere(Sphere {
//     center: array![0.0, -100.5, -1.0],
//     radius: 100.0,
//     material: &material_ground,
// });
// let sphere2 = Object::Sphere(Sphere {
//     center: array![0.0, 0.0, -1.0],
//     radius: 0.5,
//     material: &material_center,
// });
// let sphere3 = Object::Sphere(Sphere {
//     center: array![-1.0, 0.0, -1.0],
//     radius: 0.5,
//     material: &material_left,
// });
// let sphere4 = Object::Sphere(Sphere {
//     center: array![1.0, 0.0, -1.0],
//     radius: 0.5,
//     material: &material_right,
// });

// let scene_objs = vec![&sphere1, &sphere2, &sphere3, &sphere4];