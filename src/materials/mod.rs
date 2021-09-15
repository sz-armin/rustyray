use std::cmp::min_by;

use super::*;

pub enum Material {
    Diffuse(Diffuse),
    Metal(Metal),
    Glass(Glass),
    None,
}

impl Scatter for Material {
    fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> (Option<Ray>, &Array1<f64>) {
        match self {
            Material::Diffuse(diffuse) => diffuse.scatter(ray, hit_rec),
            Material::Metal(metal) => metal.scatter(ray, hit_rec),
            Material::Glass(glass) => glass.scatter(ray, hit_rec),
            Material::None => unreachable!("Should not call scatter on None material!"),
        }
    }
}

#[derive(Builder)]
pub struct Diffuse {
    #[builder(default = "array![1.0, 1.0, 1.0]")]
    pub albedo: Array1<f64>,
}

impl Scatter for Diffuse {
    fn scatter(&self, _ray: &Ray, hit_rec: &HitRecord) -> (Option<Ray>, &Array1<f64>) {
        // let mut rng = thread_rng();
        // rng.gen_bool(0.75)
        if true {
            let scattered_ray = Ray {
                direction: &hit_rec.normal + random_in_unit_sphere().unit(),
                origin: hit_rec.point.clone(),
            };
            (Some(scattered_ray), &self.albedo)
        } else {
            (None, &self.albedo)
        }
    }
}

#[derive(Builder)]
pub struct Metal {
    #[builder(default = "array![1.0, 1.0, 1.0]")]
    pub albedo: Array1<f64>,
    #[builder(default = "0.0")]
    pub fuzziness: f64,
}

impl Scatter for Metal {
    fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> (Option<Ray>, &Array1<f64>) {
        let scattered_direction = ray.direction.unit().reflect(&hit_rec.normal)
            + self.fuzziness * random_in_unit_sphere();
        if scattered_direction.dot(&hit_rec.normal) < 0.0 {
            return (None, &self.albedo);
        }
        let scattered_ray = Ray {
            direction: scattered_direction,
            origin: hit_rec.point.clone(),
        };
        (Some(scattered_ray), &self.albedo)
    }
}

#[derive(Builder)]
pub struct Glass {
    #[builder(default = "1.5")]
    pub ir: f64,
    #[builder(default = "array![1.0, 1.0, 1.0]")]
    pub albedo: Array1<f64>,
}

impl Glass {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0.powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Scatter for Glass {
    fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> (Option<Ray>, &Array1<f64>) {
        let mut rng = thread_rng();
        let irs;
        if hit_rec.front_face {
            irs = (1.0, self.ir);
        } else {
            // TODO a better way?
            irs = (self.ir * self.ir, self.ir);
        }

        let cos_theta = min_by((-ray.direction.unit()).dot(&hit_rec.normal), 1.0, |x, y| {
            x.partial_cmp(y).expect("Comparing NaN values!")
        });
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        let out_ray;
        if irs.0 / irs.1 * sin_theta > 1.0
            || Self::reflectance(cos_theta, irs.0 / irs.1) > rng.gen::<f64>()
        {
            out_ray = Ray {
                direction: ray.direction.unit().reflect(&hit_rec.normal),
                origin: hit_rec.point.clone(),
            };
        } else {
            out_ray = Ray {
                direction: ray.direction.unit().refract(&hit_rec.normal, irs),
                origin: hit_rec.point.clone(),
            };
        }
        (Some(out_ray), &self.albedo)
    }
}

impl<T: Scatter> Scatter for &T {
    fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> (Option<Ray>, &Array1<f64>) {
        (*self).scatter(ray, hit_rec)
    }
}

pub trait Scatter {
    fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> (Option<Ray>, &Array1<f64>);
}
