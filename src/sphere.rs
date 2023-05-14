use crate::{vec3::Point, hittable::{Hittable, HitRecord}, ray::Ray};

#[derive(Debug, Default)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let h = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-h - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-h + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let mut record = HitRecord::default();
        record.time = root;
        record.point = ray.at(record.time);
        let outward_normal = (record.point - self.center) / self.radius;
        record.set_face_normal(ray, &outward_normal);
        Some(record)
    }
}
