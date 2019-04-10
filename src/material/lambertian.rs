use super::random_in_unit_sphere;
use crate::prelude::*;

pub struct Lambertian {
    albedo: Col,
}

impl Lambertian {
    pub fn new(albedo: Col) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, &HitRecord { p, normal, .. }: &HitRecord) -> Option<Scatter> {
        let target = p + normal + random_in_unit_sphere();
        let scattered = Ray::new(p, target - p, r_in.time());
        let attenuation = self.albedo;
        Some(Scatter {
            scattered,
            attenuation,
        })
    }
}
