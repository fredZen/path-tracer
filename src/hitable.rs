mod bounding_box;
mod list;
mod moving_sphere;
mod prelude;
mod sphere;

use prelude::*;
use std::fmt::Debug;

pub struct HitRecord<'a> {
    pub t: Float,
    pub p: Pos,
    pub normal: Dir,
    pub mat: &'a Material,
}

pub trait Hitable: Debug {
    fn hit(&self, r: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord>;
    fn bounding_box(&self, t0: Float, t1: Float) -> Option<Cow<BoundingBox>>;
}

pub type HitableBox = Box<Hitable + Send + Sync>;

pub fn bounding_hierarchy(list: Vec<HitableBox>, time0: Float, time1: Float) -> HitableBox {
    bounding_box::BoundingHierarchy::build(list, time0, time1)
}

pub fn hitable_list(list: Vec<HitableBox>) -> HitableBox {
    Box::new(list::HitableList::new(list))
}

pub fn sphere(center: Pos, radius: Float, mat: Box<Material + Sync + Send>) -> HitableBox {
    Box::new(sphere::Sphere::new(center, radius, mat))
}

pub fn moving_sphere(
    cen0: Pos,
    cen1: Pos,
    t0: Float,
    t1: Float,
    radius: Float,
    mat: Box<Material + Sync + Send>,
) -> HitableBox {
    Box::new(moving_sphere::MovingSphere::new(
        cen0, cen1, t0, t1, radius, mat,
    ))
}
