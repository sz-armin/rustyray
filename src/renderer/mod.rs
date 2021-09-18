use super::*;

mod camera;
pub use camera::*;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn get_color<'b, 'a: 'b, T: Hit<'b, 'a>>(
        &self,
        scene_objs: &'a T,
        depth: u32,
    ) -> Vector3<f64> {
        if depth == 0 {
            return vector![0.0, 0.0, 0.0];
        }
        let mut hit_rec = HitRecord::new(&Material::None);
        // TODO Range?
        if scene_objs.hit(self, 0.001..f64::INFINITY, &mut hit_rec) {
            #[cfg(debug_assertions)]
            if NORMAL {
                return 0.5
                    * vector![
                        hit_rec.normal[0] + 1.0,
                        hit_rec.normal[1] + 1.0,
                        hit_rec.normal[2] + 1.0
                    ];
            }

            let (scattered_ray, attenuation) = hit_rec.material.scatter(self, &hit_rec);
            match scattered_ray {
                Some(mut ray) => {
                    if ray.direction.is_near_zero() {
                        ray.direction = hit_rec.normal;
                    }
                    return ray
                        .get_color(scene_objs, depth - 1)
                        .component_mul(attenuation);
                }
                None => return vector![0.0, 0.0, 0.0],
            }
        }

        let unit_dir = 0.5 * (self.direction.normalize().add_scalar(1.0));
        // Colors
        (1.0 - unit_dir[1]) * vector![1.0, 1.0, 1.0] + unit_dir[1] * vector![0.5, 0.7, 1.0]
    }

    pub fn at(&self, t: f64) -> Vector3<f64> {
        self.origin + t * self.direction
    }
}

#[derive(Clone, Debug)]
pub struct HitRecord<'a> {
    pub point: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub t: f64,
    pub front_face: bool,
    pub material: &'a Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(material: &'a Material) -> Self {
        HitRecord {
            point: Vector3::zeros(),
            normal: Vector3::zeros(),
            t: 0.0,
            front_face: true,
            material,
        }
    }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector3<f64>) {
        self.front_face = ray.direction.dot(&outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}

#[derive(Builder, Debug)]
#[builder(build_fn(skip))]
pub struct Canvas {
    pub width: u32,
    pub height: u32,
    #[builder(setter(skip))]
    pub aspect_ratio: f64,
    #[builder(setter(skip))]
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

impl CanvasBuilder {
    pub fn build(&self) -> Result<Canvas, CanvasBuilderError> {
        let width = match self.width {
            Some(ref value) => value.clone(),
            None => 600,
        };
        let height = match self.height {
            Some(ref value) => value.clone(),
            None => 300,
        };

        let aspect_ratio = width as f64 / height as f64;
        let buffer = Array3::zeros((height as usize, width as usize, 3));

        let result = Ok(Canvas {
            width,
            height,
            aspect_ratio,
            buffer,
        });
        result
    }
}
