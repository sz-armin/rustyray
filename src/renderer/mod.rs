use super::*;

#[derive(Debug)]
pub struct Ray {
    pub origin: Array1<f32>,
    pub direction: Array1<f32>,
}

impl Ray {
    pub fn get_color(&self) -> Array1<f32> {
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
        let half_b = oc.dot(&self.direction);
        let c = oc.dot(&oc) - sphere.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        } else {
            return Some((-half_b - discriminant.sqrt()) / a);
        }
    }

    fn at(&self, t: f32) -> Array1<f32> {
        &self.origin + t * &self.direction
    }
}

struct hit_record {
    point: Array1<f32>,
    normal: Array1<f32>,
    t: f32,
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

trait Unit {
    fn unit(&self) -> Array1<f32>;
}

impl Unit for Array1<f32> {
    fn unit(&self) -> Array1<f32> {
        self.clone() / (self * self).sum().sqrt()
    }
}
