use super::*;

#[derive(Debug)]
pub struct Ray {
    pub origin: Array1<f32>,
    pub direction: Array1<f32>,
}

impl Ray {
    pub fn get_color<T: Hit>(&self, scene_objs: &T) -> Array1<f32> {
        let mut hit_rec = HitRecord::new();
        if scene_objs.hit(self, 0.0..f32::INFINITY, &mut hit_rec) {
            return 0.5
                * array![
                    hit_rec.normal[0] + 1.0,
                    hit_rec.normal[1] + 1.0,
                    hit_rec.normal[2] + 1.0
                ];
        }

        let unit_dir = 0.5 * (self.direction.unit() + 1.0);
        // Colors
        (1.0 - unit_dir[1]) * array![1.0, 1.0, 1.0] + unit_dir[1] * array![0.5, 0.7, 1.0]
    }

    pub fn at(&self, t: f32) -> Array1<f32> {
        &self.origin + t * &self.direction
    }
}

#[derive(Clone)]
pub struct HitRecord {
    pub point: Array1<f32>,
    pub normal: Array1<f32>,
    pub t: f32,
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
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Array1<f32>) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal.clone();
        } else {
            self.normal = -outward_normal;
        }
    }
}

pub struct ViewPort {
    pub height: f32,
    pub width: f32,
}

pub struct Camera {
    pub viewport: ViewPort,
    pub focal_length: f32,
}

impl Camera {
    pub fn new() -> Self {
        let viewport = ViewPort {
            height: 2.0,
            width: 2.0 * 16.0 / 9.0,
        };
        Camera {
            viewport,
            focal_length: 1.0,
        }
    }
}
pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: f32,
    pub buffer: Array3<f32>,
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
    fn unit(&self) -> Array1<f32>;
}

impl Unit for Array1<f32> {
    fn unit(&self) -> Array1<f32> {
        self.clone() / (self * self).sum().sqrt()
    }
}
