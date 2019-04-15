mod bounding_box;
mod list;
mod moving_sphere;
mod prelude;
mod sphere;
mod stats;

use prelude::*;
use stats::StatsRecorder;

pub struct HitRecord<'a> {
    pub t: Float,
    pub p: Pos,
    pub normal: Dir,
    pub mat: &'a Material,
}

pub trait Hitable<C> {
    fn hit(&self, c: &mut C, r: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord>;
    fn bounding_box(&self, t0: Float, t1: Float) -> Option<Cow<BoundingBox>>;
}

pub type HitableBox<C> = Box<Hitable<C> + Send + Sync>;

impl<C> Hitable<C> for HitableBox<C> {
    fn hit(&self, c: &mut C, r: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        (&(**self)).hit(c, r, t_min, t_max)
    }

    fn bounding_box(&self, t0: Float, t1: Float) -> Option<Cow<BoundingBox>> {
        (&(**self)).bounding_box(t0, t1)
    }
}

pub trait HitableFactory<C> {
    fn bounding_hierarchy(
        &self,
        list: Vec<HitableBox<C>>,
        time0: Float,
        time1: Float,
    ) -> HitableBox<C>;
    fn hitable_list(&self, list: Vec<HitableBox<C>>) -> HitableBox<C>;
    fn sphere(&self, center: Pos, radius: Float, mat: Box<Material + Sync + Send>)
        -> HitableBox<C>;
    fn moving_sphere(
        &self,
        cen0: Pos,
        cen1: Pos,
        t0: Float,
        t1: Float,
        radius: Float,
        mat: Box<Material + Sync + Send>,
    ) -> HitableBox<C>;
}

pub struct PlainHitableFactory;

impl<C: 'static> HitableFactory<C> for PlainHitableFactory {
    fn bounding_hierarchy(
        &self,
        list: Vec<HitableBox<C>>,
        time0: Float,
        time1: Float,
    ) -> HitableBox<C> {
        bounding_box::BoundingHierarchy::build(self, list, time0, time1)
    }

    fn hitable_list(&self, list: Vec<HitableBox<C>>) -> HitableBox<C> {
        Box::new(list::HitableList::new(list))
    }

    fn sphere(
        &self,
        center: Pos,
        radius: Float,
        mat: Box<Material + Sync + Send>,
    ) -> HitableBox<C> {
        Box::new(sphere::Sphere::new(center, radius, mat))
    }

    fn moving_sphere(
        &self,
        cen0: Pos,
        cen1: Pos,
        t0: Float,
        t1: Float,
        radius: Float,
        mat: Box<Material + Sync + Send>,
    ) -> HitableBox<C> {
        Box::new(moving_sphere::MovingSphere::new(
            cen0, cen1, t0, t1, radius, mat,
        ))
    }
}

pub struct TracingHitableFactory;

pub type Stats = stats::Stats<&'static str>;
fn stats_recorder<H>(category: &'static str, hitable: H) -> HitableBox<Stats>
where
    H: 'static + Hitable<Stats> + Send + Sync,
{
    Box::new(StatsRecorder::new(category, hitable))
}

impl HitableFactory<Stats> for TracingHitableFactory {
    fn bounding_hierarchy(
        &self,
        list: Vec<HitableBox<Stats>>,
        time0: Float,
        time1: Float,
    ) -> HitableBox<Stats> {
        Box::new(stats_recorder(
            "bounding_box",
            bounding_box::BoundingHierarchy::build(self, list, time0, time1),
        ))
    }

    fn hitable_list(&self, list: Vec<HitableBox<Stats>>) -> HitableBox<Stats> {
        stats_recorder("list", list::HitableList::new(list))
    }

    fn sphere(
        &self,
        center: Pos,
        radius: Float,
        mat: Box<Material + Sync + Send>,
    ) -> HitableBox<Stats> {
        stats_recorder("sphere", sphere::Sphere::new(center, radius, mat))
    }

    fn moving_sphere(
        &self,
        cen0: Pos,
        cen1: Pos,
        t0: Float,
        t1: Float,
        radius: Float,
        mat: Box<Material + Sync + Send>,
    ) -> HitableBox<Stats> {
        stats_recorder(
            "moving_sphere",
            moving_sphere::MovingSphere::new(cen0, cen1, t0, t1, radius, mat),
        )
    }
}
