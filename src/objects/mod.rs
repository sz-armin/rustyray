use super::*;

pub struct Sphere {
    pub center: Array1<f32>,
    pub radius: f32,
}

trait Object {
    fn hit(&self, ray: Ray) -> f32;
}