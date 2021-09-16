use super::*;
use ndarray_rand::rand_distr::{Distribution, Uniform};
use ndarray_rand::RandomExt;

pub fn random_in_unit_sphere() -> Array1<f64> {
    loop {
        let array = Array1::random(3, Uniform::new(-1.0, 1.0));
        if array.dot(&array) < 1.0 {
            return array;
        }
    }
}

pub fn random_in_unit_disk() -> Array1<f64> {
    let mut rng = thread_rng();
    let between = Uniform::new(-1.0, 1.0);
    loop {
        let array = array![between.sample(&mut rng), between.sample(&mut rng), 0.0];
        if array.dot(&array) < 1.0 {
            return array;
        }
    }
}
