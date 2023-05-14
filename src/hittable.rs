pub mod hit_list;
pub mod sphere;

use std::rc::Rc;
use crate::{material::Material, ray::Ray, vec3::*};

#[derive(Debug)]
pub struct HitRecord {
    pub point: Point,
    pub normal: Vec3,
    pub time: f64,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        point: Point,
        time: f64,
        material: Rc<dyn Material>,
        ray: &Ray,
        outward_normal: &Vec3,
    ) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        Self {
            point,
            time,
            material,
            front_face,
            normal: if front_face {
                *outward_normal
            } else {
                -*outward_normal
            },
        }
    }
}

pub trait Hittable: std::fmt::Debug {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
