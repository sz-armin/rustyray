use super::*;

pub fn random_in_unit_sphere() -> Vector3<f64> {
    let mut rng = thread_rng();
    loop {
        let array = Vector3::from_distribution(&Uniform::new(-1.0, 1.0), &mut rng);
        if array.dot(&array) < 1.0 {
            return array;
        }
    }
}

pub fn random_in_unit_disk() -> Vector3<f64> {
    let mut rng = thread_rng();
    let between = Uniform::new(-1.0, 1.0);
    loop {
        let array = vector![between.sample(&mut rng), between.sample(&mut rng), 0.0];
        if array.dot(&array) < 1.0 {
            return array;
        }
    }
}
