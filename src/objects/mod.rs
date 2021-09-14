use super::*;
use std::{borrow::Borrow, mem::ManuallyDrop};

pub enum Object<'a> {
    Sphere(Sphere<'a>),
}

impl<'b, 'a: 'b> Hit<'b, 'a> for Object<'a> {
    fn hit(&self, ray: &Ray, t_range: std::ops::Range<f64>, hit_rec: &mut HitRecord<'b>) -> bool {
        match self {
            Object::Sphere(sphere) => sphere.hit(ray, t_range, hit_rec),
        }
    }
}

pub struct Sphere<'a> {
    pub center: Array1<f64>,
    pub radius: f64,
    pub material: &'a Material,
}

impl<'b, 'a: 'b> Hit<'b, 'a> for Sphere<'a> {
    fn hit(&self, ray: &Ray, t_range: std::ops::Range<f64>, hit_rec: &mut HitRecord<'b>) -> bool {
        let oc = &ray.origin - &self.center;
        let a = ray.direction.dot(&ray.direction);
        let half_b = oc.dot(&ray.direction);
        // TODO i?
        let c = oc.dot(&oc) - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let mut root = (-half_b - discriminant.sqrt()) / a;
        if !(t_range.contains(&root)) {
            root = (-half_b + discriminant.sqrt()) / a;
            if !(t_range.contains(&root)) {
                return false;
            }
        }

        // TODO only calculate when needed
        hit_rec.t = root;
        hit_rec.point = ray.at(hit_rec.t);
        let outward_normal = (&hit_rec.point - &self.center) / self.radius;
        hit_rec.set_face_normal(ray, &outward_normal);
        hit_rec.material = self.material;

        true
    }
}

impl<'b, 'a: 'b, T: Hit<'b, 'a>> Hit<'b, 'a> for Vec<T> {
    fn hit<'c>(
        &self,
        ray: &Ray,
        t_range: std::ops::Range<f64>,
        hit_rec: &mut HitRecord<'b>,
    ) -> bool {
        // is there a better way?
        // let objs_refs: Vec<_> = self.iter().collect();
        let mut temp_rec = HitRecord::new(&Material::None);
        let mut hit_anything = false;
        let mut closet = t_range.end;
        // TODO is there a better way?
        for obj in self {
            if obj.hit(ray, t_range.start..closet, &mut temp_rec) {
                hit_anything = true;
                closet = temp_rec.t;
                *hit_rec = temp_rec.clone();
            }
        }
        hit_anything
    }
}

// impl Hit for Vec<&Object> {
//     fn hit(&self, ray: &Ray, t_range: std::ops::Range<f64>, hit_rec: &mut HitRecord) -> bool {
//         let mut temp_rec = HitRecord::new();
//         let mut hit_anything = false;
//         let mut closet = t_range.end;
//         // TODO is there a better way?
//         for obj in self {
//             if obj.hit(ray, 0.0..closet, &mut temp_rec) {
//                 hit_anything = true;
//                 closet = temp_rec.t;
//                 *hit_rec = temp_rec.clone();
//             }
//         }
//         hit_anything
//     }
// }

impl<'b, 'a: 'b, T: Hit<'b, 'a>> Hit<'b, 'a> for &'_ T {
    fn hit(&self, ray: &Ray, t_range: std::ops::Range<f64>, hit_rec: &mut HitRecord<'b>) -> bool {
        (*self).hit(ray, t_range, hit_rec)
    }
}

pub trait Hit<'b, 'a: 'b> {
    fn hit(&self, ray: &Ray, t_range: std::ops::Range<f64>, hit_rec: &mut HitRecord<'b>) -> bool;
}
