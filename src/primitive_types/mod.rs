use super::*;

impl IsNearZero for Array1<f64> {
    fn is_near_zero(&self) -> bool {
        Zip::from(self).all(|x| x.abs() < 0.00000001)
    }
}

impl IsNearZero for Vector3<f64> {
    fn is_near_zero(&self) -> bool {
        // TODO
        self.iter().all(|x| x.abs() < 0.00000001)
    }
}

impl<T: IsNearZero> IsNearZero for &T {
    fn is_near_zero(&self) -> bool {
        (*self).is_near_zero()
    }
}

impl Reflect for Array1<f64> {
    fn reflect(&self, criteria: &Array1<f64>) -> Array1<f64> {
        self - 2.0 * self.dot(criteria) * criteria
    }
}

impl Reflect for Vector3<f64> {
    fn reflect(&self, criteria: &Vector3<f64>) -> Vector3<f64> {
        self - 2.0 * self.dot(criteria) * criteria
    }
}

// impl<T: Reflect> Reflect for &T {
//     fn reflect(&self, criteria: &Array1<f64>) -> Array1<f64> {
//         (*self).reflect(criteria)
//     }
// }

impl Refract for Array1<f64> {
    fn refract(&self, normal: &Array1<f64>, irs: (f64, f64)) -> Array1<f64> {
        let refraction_ratio = irs.0 / irs.1;
        let cos_theta = std::cmp::min_by((-self).dot(normal), 1.0, |x, y| {
            x.partial_cmp(y).expect("Comparing NaN values!")
        });
        let r_out_perp = refraction_ratio * (self + cos_theta * normal);
        let r_out_parallel = -((1.0 - r_out_perp.dot(&r_out_perp)).abs()).sqrt() * normal;
        r_out_perp + r_out_parallel
    }
}

impl Refract for Vector3<f64> {
    fn refract(&self, normal: &Vector3<f64>, irs: (f64, f64)) -> Vector3<f64> {
        let refraction_ratio = irs.0 / irs.1;
        let cos_theta = std::cmp::min_by((-self).dot(normal), 1.0, |x, y| {
            x.partial_cmp(y).expect("Comparing NaN values!")
        });
        let r_out_perp = refraction_ratio * (self + cos_theta * normal);
        let r_out_parallel = -((1.0 - r_out_perp.dot(&r_out_perp)).abs()).sqrt() * normal;
        r_out_perp + r_out_parallel
    }
}

// impl<T: Refract> Refract for &T {
//     fn refract(&self, normal: &T, irs: (f64, f64)) -> T {
//         (*self).refract(normal, irs)
//     }
// }

impl Unit for Array1<f64> {
    fn unit(&self) -> Array1<f64> {
        self.clone() / self.dot(self).sqrt()
    }
}

impl Cross for Array1<f64> {
    fn cross(&self, b: &Array1<f64>) -> Array1<f64> {
        let rx = self[1] * b[2] - self[2] * b[1];
        let ry = self[2] * b[0] - self[0] * b[2];
        let rz = self[0] * b[1] - self[1] * b[0];
        array![rx, ry, rz]
    }
}

pub trait IsNearZero {
    fn is_near_zero(&self) -> bool;
}

pub trait Reflect {
    fn reflect(&self, criteria: &Self) -> Self;
}

pub trait Refract {
    fn refract(&self, normal: &Self, irs: (f64, f64)) -> Self;
}

pub trait Unit {
    fn unit(&self) -> Array1<f64>;
}

pub trait Cross {
    fn cross(&self, array2: &Array1<f64>) -> Array1<f64>;
}
