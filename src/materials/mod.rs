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

pub struct Glass {
    pub ir: f64,
    pub albedo: Array1<f64>,
}

impl Scatter for Glass {
    fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> (Option<Ray>, &Array1<f64>) {
        let attenuation = array![1.0, 1.0, 1.0];
        // lrt refraction_ratio = rec.front_face ? (1.0/ir) : ir;
        let irs;
        if hit_rec.front_face {
            irs = (1.0, self.ir);
        } else {
            // TODO a better way?
            irs = (self.ir * self.ir, self.ir);
        }
        let out_ray = Ray {
            direction: ray.direction.unit().refract(&hit_rec.normal, irs),
            origin: hit_rec.point.clone(),
        };
        (Some(out_ray), &self.albedo)

        // vec3 unit_direction = unit_vector(r_in.direction());
        // vec3 refracted = refract(unit_direction, rec.normal, refraction_ratio);

        // scattered = ray(rec.p, refracted);
        // return true;
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
