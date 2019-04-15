use super::prelude::*;

pub struct HitableList<C> {
    list: Vec<Box<Hitable<C> + Send + Sync>>,
}

impl<C> HitableList<C> {
    pub fn new(list: Vec<Box<Hitable<C> + Send + Sync>>) -> Self {
        Self { list }
    }
}

impl<C> Hitable<C> for HitableList<C> {
    fn hit(&self, c: &mut C, r: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let mut result = None;
        let mut closest_so_far = t_max;

        for h in self.list.iter() {
            let res = h.hit(c, r, t_min, closest_so_far);
            if let Some(HitRecord { t, .. }) = res {
                closest_so_far = t;
                result = res;
            }
        }

        result
    }

    fn bounding_box(&self, t0: Float, t1: Float) -> Option<Cow<BoundingBox>> {
        let first = self.list.first()?.bounding_box(t0, t1)?;

        let r: Result<_, ()> = self.list.iter().skip(1).try_fold(first, |bounds, hitable| {
            Ok(Cow::Owned(BoundingBox::surrounding(
                bounds,
                hitable.bounding_box(t0, t1).ok_or(())?,
            )))
        });

        r.ok()
    }
}
