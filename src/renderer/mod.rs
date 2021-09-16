use std::{default, io::ErrorKind};

use super::*;

use std::pin::Pin;

#[derive(Debug)]
pub struct Ray {
    pub origin: Array1<f64>,
    pub direction: Array1<f64>,
}

impl Ray {
    pub fn get_color<'b, 'a: 'b, T: Hit<'b, 'a>>(
        &self,
        scene_objs: &'a T,
        depth: u32,
    ) -> Array1<f64> {
        if depth == 0 {
            return array![0.0, 0.0, 0.0];
        }
        let mut hit_rec = HitRecord::new(&Material::None);
        // TODO Range?
        if scene_objs.hit(self, 0.001..f64::INFINITY, &mut hit_rec) {
            #[cfg(debug_assertions)]
            if NORMAL {
                return 0.5
                    * array![
                        hit_rec.normal[0] + 1.0,
                        hit_rec.normal[1] + 1.0,
                        hit_rec.normal[2] + 1.0
                    ];
            }

            let (scattered_ray, attenuation) = hit_rec.material.scatter(self, &hit_rec);
            match scattered_ray {
                Some(mut ray) => {
                    if ray.direction.is_near_zero() {
                        ray.direction.assign(&hit_rec.normal);
                    }
                    return attenuation * ray.get_color(scene_objs, depth - 1);
                }
                None => return array![0.0, 0.0, 0.0],
            }
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
pub struct HitRecord<'a> {
    pub point: Array1<f64>,
    pub normal: Array1<f64>,
    pub t: f64,
    pub front_face: bool,
    pub material: &'a Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(material: &'a Material) -> Self {
        HitRecord {
            point: Array1::zeros(3),
            normal: Array1::zeros(3),
            t: 0.0,
            front_face: true,
            material: material,
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
pub struct Camera {
    #[builder(default = "array![0.0, 0.0, 0.0]")]
    pub origin: Array1<f64>,
    #[builder(default = "array![0.0, 0.0, -1.0]")]
    pub look_at: Array1<f64>,
    #[builder(default = "array![0.0, 1.0, 0.0]")]
    pub vup: Array1<f64>,

    #[builder(setter(skip))]
    pub view_height: f64,
    #[builder(setter(skip))]
    pub view_width: f64,
    #[builder(default = "90.0")]
    pub vfov: f64, // Vertical field of view (in degrees)
    #[builder(default = "16.0 / 9.0")]
    pub aspect_ratio: f64,
    #[builder(default = "1.0")]
    pub focal_length: f64,

    #[builder(default = "2.0")]
    pub aperture: f64,
    pub focus_dist: f64,
    #[builder(default = "1.0")]
    #[builder(setter(skip))]
    lens_radius: f64,

    #[builder(setter(skip))]
    pub w: Array1<f64>,
    #[builder(setter(skip))]
    pub u: Array1<f64>,
    #[builder(setter(skip))]
    pub v: Array1<f64>,

    #[builder(setter(skip))]
    pub vertical: Array1<f64>,
    #[builder(setter(skip))]
    pub horizontal: Array1<f64>,
    #[builder(setter(skip))]
    pub top_left_corner: Array1<f64>,
}

impl Camera {
    pub fn finalize_build(mut self) -> Self {
        self.w = (&self.origin - &self.look_at).unit();
        self.u = self.vup.cross(&self.w).unit();
        self.v = self.w.cross(&self.u);

        let h = (self.vfov.to_radians() / 2.0).tan();
        self.view_height = 2.0 * h;
        self.view_width = self.view_height * self.aspect_ratio;

        self.horizontal = self.focus_dist * self.view_width * &self.u;
        self.vertical = self.focus_dist * self.view_height * &self.v;
        self.top_left_corner =
            &self.origin - (&self.horizontal / 2.0) + (&self.vertical / 2.0) - self.focus_dist * &self.w;

        self.lens_radius = self.aperture / 2.0;

        self
    }
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = &self.u * rd[0] + &self.v * rd[1];

        Ray {
            origin: self.origin.clone() + &offset,
            direction: (&self.top_left_corner + s * &self.horizontal
                - t * &self.vertical
                - &self.origin - &offset),
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        CameraBuilder::default().build().unwrap().finalize_build()
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