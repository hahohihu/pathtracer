pub mod dielectric;
pub mod lambertian;
pub mod metal;

use crate::{common::*, hittable::HitRecord};

pub struct ScatterRecord {
    pub scattered: Ray,
    pub attenuation: Color,
}

pub trait Material: std::fmt::Debug + Send + Sync {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord>;
}
