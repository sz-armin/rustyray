use super::*;
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;

pub fn random_in_unit_sphere() -> Array1<f64> {
    loop {
        let array = Array1::random(3, Uniform::new(-1.0, 1.0));
        if array.dot(&array) < 1.0 {
            return array;
        }
    }
}
