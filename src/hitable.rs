mod list;
mod sphere;

pub use list::HitableList;
pub use sphere::Sphere;

use crate::ray::Ray;
use crate::vec3::{Dir, Float, Pos};

pub struct HitRecord {
    pub t: Float,
    pub p: Pos,
    pub normal: Dir,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord>;
}
