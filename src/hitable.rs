mod list;
mod sphere;

use crate::prelude::*;

pub struct HitRecord<'a> {
    pub t: Float,
    pub p: Pos,
    pub normal: Dir,
    pub mat: &'a Material,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord>;
}

pub type HitableBox = Box<Hitable + Send + Sync>;

pub fn hitable_list(list: Vec<HitableBox>) -> HitableBox {
    Box::new(list::HitableList::new(list))
}

pub fn sphere(center: Pos, radius: Float, mat: Box<Material + Sync + Send>) -> HitableBox {
    Box::new(sphere::Sphere::new(center, radius, mat))
}
