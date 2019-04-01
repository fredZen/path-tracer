use super::{Material, Scatter};
use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Col, Dir, Vector};
use rand::prelude::*;

pub struct Lambertian {
    albedo: Col,
}

fn random_in_unit_sphere() -> Dir {
    let mut rng = thread_rng();
    loop {
        let p = 2. * Dir::new(rng.gen(), rng.gen(), rng.gen()) - Dir::new(1., 1., 1.);
        if p.squared_length() < 1. {
            return p;
        }
    }
}

impl Lambertian {
    pub fn new(albedo: Col) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, &HitRecord { p, normal, .. }: &HitRecord) -> Option<Scatter> {
        let target = p + normal + random_in_unit_sphere();
        let scattered = Ray::new(p, target - p);
        let attenuation = self.albedo;
        Some(Scatter {
            scattered,
            attenuation,
        })
    }
}
