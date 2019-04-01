mod list;
mod sphere;

pub use list::HitableList;
pub use sphere::Sphere;

use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Dir, Float, Pos};

pub struct HitRecord<'a> {
    pub t: Float,
    pub p: Pos,
    pub normal: Dir,
    pub mat: &'a Material,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord>;
}
