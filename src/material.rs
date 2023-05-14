pub mod lambertian;
pub mod metal;

use crate::{rt_weekend::*, hittable::HitRecord};

pub struct ScatterRecord {
    pub scattered: Ray,
    pub attenuation: Color,
}

pub trait Material: std::fmt::Debug {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord>;
}
