use crate::prelude::*;

pub struct MovingSphere {
    center0: Pos,
    center1: Pos,
    time0: Float,
    time1: Float,
    radius: Float,
    mat: Box<Material + Send + Sync>,
}

impl MovingSphere {
    pub fn new(
        cen0: Pos,
        cen1: Pos,
        t0: Float,
        t1: Float,
        radius: Float,
        mat: Box<Material + Sync + Send>,
    ) -> Self {
        Self {
            center0: cen0,
            center1: cen1,
            time0: t0,
            time1: t1,
            radius,
            mat,
        }
    }

    #[inline]
    fn center(&self, time: Float) -> Pos {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }

    #[inline]
    fn hit_record(&self, r: &Ray, t: Float) -> HitRecord {
        let p = r.point_at(t);
        HitRecord {
            t,
            p,
            normal: (p - self.center(r.time())) / self.radius,
            mat: &*self.mat,
        }
    }
}

impl Hitable for MovingSphere {
    #[inline]
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let oc = ray.origin() - self.center(ray.time());
        let a = ray.direction().squared_length();
        let b = oc.dot(ray.direction());
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0. {
            let t = (-b - discriminant.sqrt()) / a;
            if t_min < t && t < t_max {
                return Some(self.hit_record(ray, t));
            }
            let t = (-b + discriminant.sqrt()) / a;
            if t_min < t && t < t_max {
                return Some(self.hit_record(ray, t));
            }
        }
        None
    }
}
