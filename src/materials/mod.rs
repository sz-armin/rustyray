use super::*;

pub enum Material {
    Lambertian(Lambertian),
    None
}

pub struct Lambertian {

}