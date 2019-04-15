use super::prelude::*;

#[derive(Debug)]
pub struct Sphere {
    center: Pos,
    radius: Float,
    mat: Box<Material + Send + Sync>,
}

impl Sphere {
    pub fn new(center: Pos, radius: Float, mat: Box<Material + Sync + Send>) -> Sphere {
        Sphere {
            center,
            radius,
            mat,
        }
    }

    #[inline]
    fn hit_record(&self, r: &Ray, t: Float) -> HitRecord {
        let p = r.point_at(t);
        HitRecord {
            t,
            p,
            normal: (p - self.center) / self.radius,
            mat: &*self.mat,
        }
    }
}

impl<C> Hitable<C> for Sphere {
    #[inline]
    fn hit(&self, _c: &mut C, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
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

    fn bounding_box(&self, _t0: Float, _t1: Float) -> Option<Cow<BoundingBox>> {
        let half_diag = dir(self.radius, self.radius, self.radius);
        Some(Cow::Owned(BoundingBox::new(
            self.center - half_diag,
            self.center + half_diag,
        )))
    }
}
