use super::*;

pub enum Material {
    Diffuse(Diffuse),
    None,
}

impl Scatter for Material {
    fn scatter(&self, hit_rec: &HitRecord) -> (Ray, &Array1<f64>) {
        match self {
            Material::Diffuse(diffuse) => diffuse.scatter(hit_rec),
            Material::None => unreachable!(),
        }
    }
}

pub struct Diffuse {
    pub albedo: Array1<f64>,
}

impl Scatter for Diffuse {
    fn scatter(&self, hit_rec: &HitRecord) -> (Ray, &Array1<f64>) {
        let scattered_ray = Ray {
            direction: &hit_rec.normal + random_in_unit_circle().unit(),
            // direction : &self.direction - 2.0*self.direction.dot(&hit_rec.normal)*&hit_rec.normal,
            origin: hit_rec.point.clone(),
        };
        (scattered_ray, &self.albedo)
    }
}



impl<T: Scatter> Scatter for &T {
    fn scatter(&self, hit_rec: &HitRecord) -> (Ray, &Array1<f64>) {
        (*self).scatter(hit_rec)
    }
}

pub trait Scatter {
    fn scatter(&self, hit_rec: &HitRecord) -> (Ray, &Array1<f64>);
}
