use crate::{
    hittable::HitRecord,
    material::{Material, ScatterRecord},
    rt_weekend::*,
};

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        Some(ScatterRecord {
            scattered: Ray::new(rec.point, scatter_direction),
            attenuation: self.albedo,
        })
    }
}
