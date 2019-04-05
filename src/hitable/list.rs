use super::{HitRecord, Hitable};
use crate::ray::Ray;
use crate::vec3::Float;

pub struct HitableList {
    list: Vec<Box<Hitable + Send + Sync>>,
}

impl HitableList {
    pub fn new(list: Vec<Box<Hitable + Send + Sync>>) -> HitableList {
        HitableList { list }
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let mut result = None;
        let mut closest_so_far = t_max;

        for h in self.list.iter() {
            let res = h.hit(r, t_min, closest_so_far);
            if let Some(HitRecord { t, .. }) = res {
                closest_so_far = t;
                result = res;
            }
        }

        result
    }
}
