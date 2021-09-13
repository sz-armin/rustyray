use std::{default, io::ErrorKind};

use super::*;

use std::pin::Pin;

#[derive(Debug)]
pub struct Ray {
    pub origin: Array1<f64>,
    pub direction: Array1<f64>,
}

impl Ray {
    pub fn get_color<T: Hit>(&self, scene_objs: &T, depth: u32) -> Array1<f64> {
        if depth == 0 {
            return array![0.0, 0.0, 0.0];
        }
        let mut hit_rec = HitRecord::new();
        if scene_objs.hit(self, f64::EPSILON..f64::INFINITY, &mut hit_rec) {
            // Normal
            // return 0.5
            //     * array![
            //         hit_rec.normal[0] + 1.0,
            //         hit_rec.normal[1] + 1.0,
            //         hit_rec.normal[2] + 1.0
            //     ];

            // TODO Optimize
            let target_point = &hit_rec.normal + &hit_rec.point + random_in_unit_circle();
            let ray = Ray {
                direction: target_point - &hit_rec.point,
                // direction : &self.direction - 2.0*self.direction.dot(&hit_rec.normal)*&hit_rec.normal,
                origin: hit_rec.point,
            };
            // TODO why 0.5?
            return 0.5 * ray.get_color(scene_objs, depth - 1);
        }

        let unit_dir = 0.5 * (self.direction.unit() + 1.0);
        // Colors
        (1.0 - unit_dir[1]) * array![1.0, 1.0, 1.0] + unit_dir[1] * array![0.5, 0.7, 1.0]
    }

    pub fn at(&self, t: f64) -> Array1<f64> {
        &self.origin + t * &self.direction
    }
}

#[derive(Clone)]
pub struct HitRecord {
    pub point: Array1<f64>,
    pub normal: Array1<f64>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            point: Array1::zeros(3),
            normal: Array1::zeros(3),
            t: 0.0,
            front_face: true,
        }
    }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Array1<f64>) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal.clone();
        } else {
            self.normal = -outward_normal;
        }
    }
}

#[derive(Builder, Clone)]
pub struct ViewPort {
    #[builder(default = "2.0")]
    pub height: f64,
    #[builder(default = "3.55")]
    pub width: f64,
    #[builder(default = "1.0")]
    pub focal_length: f64,
    #[builder(default = "array![0.0, 0.0, 0.0]")]
    pub origin: Array1<f64>,
    #[builder(setter(skip))]
    pub vertical: Array1<f64>,
    #[builder(setter(skip))]
    pub horizontal: Array1<f64>,
    #[builder(setter(skip))]
    pub top_left_corner: Array1<f64>,
}

impl ViewPort {
    fn finalize_build(mut self) -> Self {
        self.horizontal = array![self.width, 0.0, 0.0];
        self.vertical = array![0.0, self.height, 0.0];
        self.top_left_corner = &self.origin - (&self.horizontal / 2.0) + (&self.vertical / 2.0)
            - array![0.0, 0.0, self.focal_length];
        self
    }
}

impl Default for ViewPort {
    fn default() -> Self {
        ViewPortBuilder::default().build().unwrap().finalize_build()
    }
}

#[derive(Builder)]
pub struct Camera {
    pub viewport: ViewPort,
}

impl Default for Camera {
    fn default() -> Self {
        CameraBuilder::default()
            .viewport(ViewPort::default())
            .build()
            .unwrap()
    }
}

pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: f64,
    pub buffer: Array3<f64>,
}

impl Canvas {
    pub fn save(&self) -> Result<(), ImageError> {
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

pub trait Unit {
    fn unit(&self) -> Array1<f64>;
}

impl Unit for Array1<f64> {
    fn unit(&self) -> Array1<f64> {
        self.clone() / (self * self).sum().sqrt()
    }
}
