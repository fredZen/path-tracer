use super::prelude::*;

#[derive(Debug)]
pub struct HitableList {
    list: Vec<Box<Hitable + Send + Sync>>,
}

impl HitableList {
    pub fn new(list: Vec<Box<Hitable + Send + Sync>>) -> Self {
        Self { list }
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
