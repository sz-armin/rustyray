use super::*;

impl IsNearZero for Array1<f64> {
    fn is_near_zero(&self) -> bool {
        Zip::from(self).all(|x| x.abs() < 0.00000001)
    }
}

impl<T: IsNearZero>IsNearZero for &T {
    fn is_near_zero(&self) -> bool {
        (*self).is_near_zero()
    }
}

impl Reflect for Array1<f64> {
    fn reflect(&self,criteria: &Array1<f64>) -> Array1<f64> {
        self - 2.0 * self.dot(criteria) * criteria
    }
}

impl<T: Reflect>Reflect for &T {
    fn reflect(&self,criteria: &Array1<f64>) -> Array1<f64> {
        (*self).reflect(criteria)
    }
}

pub trait IsNearZero {
    fn is_near_zero(&self) -> bool;
}


pub trait Reflect {
    fn reflect(&self, criteria: &Array1<f64>) -> Array1<f64>;
}