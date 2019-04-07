use super::{random_in_unit_sphere, Material, Scatter};
use crate::hitable::HitRecord;
use crate::prelude::*;
use crate::ray::Ray;

pub struct Lambertian {
    albedo: Col,
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
