// Note: For the moment, this rather seems to slow things down than speed them up

use super::prelude::*;
use crate::hitable::HitableBox;
use itertools::izip;
use rand::prelude::*;
use std::cmp::Ordering;
use std::ops::Deref;

#[derive(Clone, Debug, PartialEq)]
pub struct BoundingBox {
    min: Pos,
    max: Pos,
}

impl BoundingBox {
    pub fn new(min: Pos, max: Pos) -> Self {
        Self { min, max }
    }

    pub fn surrounding<B0: Deref<Target = Self>, B1: Deref<Target = Self>>(
        box0: B0,
        box1: B1,
    ) -> Self {
        Self {
            min: pos(
                box0.min.x().min(box1.min.x()),
                box0.min.y().min(box1.min.y()),
                box0.min.z().min(box1.min.z()),
            ),
            max: pos(
                box0.max.x().max(box1.max.x()),
                box0.max.y().max(box1.max.y()),
                box0.max.z().max(box1.max.z()),
            ),
        }
    }

    #[inline]
    fn hit(&self, r: &Ray, t_min: Float, t_max: Float) -> bool {
        for (min, max, d, o) in izip!(
            self.min.iter(),
            self.max.iter(),
            r.direction().iter(),
            r.origin().iter()
        ) {
            let inv_d = 1. / d;
            let t0 = (min - o) * inv_d;
            let t1 = (max - o) * inv_d;
            let (t0, t1) = if inv_d < 0. { (t1, t0) } else { (t0, t1) };
            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 < t_max { t1 } else { t_max };
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}

#[derive(Debug)]
pub struct BoundingHierarchy {
    bounds: BoundingBox,
    left: HitableBox,
    right: HitableBox,
}

#[derive(Clone, Copy, Debug)]
enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    fn get(self, v: Pos) -> Float {
        use Axis::*;
        match self {
            X => v.x(),
            Y => v.y(),
            Z => v.z(),
        }
    }
}

impl Distribution<Axis> for rand::distributions::Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Axis {
        use Axis::*;
        match rng.gen_range(0, 3) {
            0 => X,
            1 => Y,
            _ => Z,
        }
    }
}

fn bounding_box_compare(
    axis: Axis,
    time0: Float,
    time1: Float,
    h1: &HitableBox,
    h2: &HitableBox,
) -> Ordering {
    let c1 = axis.get(h1.bounding_box(time0, time1).expect("no bounding box").min);
    let c2 = axis.get(h2.bounding_box(time0, time1).expect("no bounding box").min);

    c1.partial_cmp(&c2).unwrap()
}

impl BoundingHierarchy {
    pub fn build(
        mut list: Vec<HitableBox>,
        time0: Float,
        time1: Float,
    ) -> HitableBox {
        let n = list.len();
        if n == 1 {
            list.into_iter().next().unwrap()
        } else {
            let (left, right) = if n == 2 {
                let mut it = list.into_iter();
                let left = it.next().unwrap();
                let right = it.next().unwrap();

                (left, right)
            } else {
                let mut rng = thread_rng();
                let axis = rng.gen();
                list.sort_unstable_by(|h1, h2| bounding_box_compare(axis, time0, time1, h1, h2));
                let tail = list.split_off(n / 2);
                let left = BoundingHierarchy::build(list, time0, time1);
                let right = BoundingHierarchy::build(tail, time0, time1);
                (left, right)
            };
            let lbounds = left.bounding_box(time0, time1).expect("no bounding box");
            let rbounds = right.bounding_box(time0, time1).expect("no bounding box");
            let bounds = BoundingBox::surrounding(lbounds, rbounds);
            Box::new(Self {
                left,
                right,
                bounds,
            })
        }
    }
}

impl Hitable for BoundingHierarchy {
    #[inline]
    fn hit(&self, r: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        if self.bounds.hit(r, t_min, t_max) {
            let left = self.left.hit(r, t_min, t_max);
            let right = self.right.hit(r, t_min, t_max);
            match (&left, &right) {
                (Some(l), Some(r)) => {
                    if l.t < r.t {
                        left
                    } else {
                        right
                    }
                }
                (None, _) => right,
                _ => left,
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, _t0: Float, _t1: Float) -> Option<Cow<BoundingBox>> {
        Some(Cow::Borrowed(&self.bounds))
    }
}
