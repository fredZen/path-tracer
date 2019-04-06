use super::{HitRecord, Hitable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Float, Pos, Vector};

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
}

impl Hitable for Sphere {
    #[inline]
    fn hit(&self, r: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(r.direction());
        let b = oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0. {
            let t = (-b - discriminant.sqrt()) / a;
            let p = r.point_at(t);
            if t_min < t && t < t_max {
                return Some(HitRecord {
                    t,
                    p,
                    normal: (p - self.center) / self.radius,
                    mat: &*self.mat,
                });
            }
            let t = (-b + discriminant.sqrt()) / a;
            let p = r.point_at(t);
            if t_min < t && t < t_max {
                return Some(HitRecord {
                    t,
                    p,
                    normal: (p - self.center) / self.radius,
                    mat: &*self.mat,
                });
            }
        }
        None
    }
}
