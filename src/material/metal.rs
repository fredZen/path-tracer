use super::{Material, Scatter};
use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Col, Dir, Vector};

pub struct Metal {
    albedo: Col,
}

impl Metal {
    pub fn new(albedo: Col) -> Self {
        Self { albedo }
    }
}

fn reflect(v: Dir, n: Dir) -> Dir {
    v - 2. * v.dot(n) * n
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, &HitRecord { p, normal, .. }: &HitRecord) -> Option<Scatter> {
        let reflected = reflect(r_in.direction().unit_vector(), normal);

        if reflected.dot(normal) > 0. {
            let scattered = Ray::new(p, reflected);
            let attenuation = self.albedo;

            Some(Scatter {
                scattered,
                attenuation,
            })
        } else {
            None
        }
    }
}
