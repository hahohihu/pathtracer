use crate::{common::*, hittable::HitRecord};

use super::{Material, ScatterRecord};

#[derive(Debug)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = Color::white();
        let refraction_ratio = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray_in.direction.unit_vec();
        let refracted = unit_direction.refract(&rec.normal, refraction_ratio);

        Some(ScatterRecord {
            scattered: Ray::new(rec.point, refracted),
            attenuation,
        })
    }
}
