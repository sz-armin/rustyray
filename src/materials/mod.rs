use super::*;

pub enum Material {
    Diffuse(Diffuse),
    Metal(Metal),
    None,
}

impl Scatter for Material {
    fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> (Option<Ray>, &Array1<f64>) {
        match self {
            Material::Diffuse(diffuse) => diffuse.scatter(ray, hit_rec),
            Material::Metal(metal) => metal.scatter(ray, hit_rec),
            Material::None => unreachable!(),
        }
    }
}

pub struct Diffuse {
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

pub struct Metal {
    pub albedo: Array1<f64>,
    pub fuzziness: f64,
}

impl Scatter for Metal {
    fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> (Option<Ray>, &Array1<f64>) {
        let scattered_direction =
            ray.direction.reflect(&hit_rec.normal) + self.fuzziness * random_in_unit_sphere();
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

impl<T: Scatter> Scatter for &T {
    fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> (Option<Ray>, &Array1<f64>) {
        (*self).scatter(ray, hit_rec)
    }
}

pub trait Scatter {
    fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> (Option<Ray>, &Array1<f64>);
}
