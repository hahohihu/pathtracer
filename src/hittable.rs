use crate::{vec3::*, ray::Ray};

pub struct HitRecord {
    pub point: Point,
    pub normal: Vec3,
    pub time: f64,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>; 
}

