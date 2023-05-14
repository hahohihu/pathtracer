use crate::hittable::*;
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct HitList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HitList {
    pub fn new(object: Arc<dyn Hittable>) -> Self {
        Self {
            objects: vec![object],
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HitList {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut res = None;
        let mut closest = t_max;
        for object in &self.objects {
            if let Some(record) = object.hit(ray, t_min, closest) {
                closest = record.time;
                res = Some(record);
            }
        }
        res
    }
}
