use crate::{
    common::*,
    material::{Material, ScatterRecord},
};

#[derive(Debug)]
pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &crate::hittable::HitRecord) -> Option<ScatterRecord> {
        let reflected = ray_in.direction.unit_vec().reflect(&rec.normal);
        let scattered = Ray::new(rec.point, reflected);
        if scattered.direction.dot(&rec.normal) > 0.0 {
            Some(ScatterRecord {
                scattered,
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}
