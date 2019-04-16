use super::random_in_unit_sphere;
use crate::prelude::*;

#[derive(Debug)]
pub struct Lambertian {
    albedo: TextureBox,
}

impl Lambertian {
    pub fn new(albedo: TextureBox) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, &HitRecord { p, normal, .. }: &HitRecord) -> Option<Scatter> {
        let target = p + normal + random_in_unit_sphere();
        let scattered = Ray::new(p, target - p, r_in.time());
        let attenuation = self.albedo.value(0., 0., &p);
        Some(Scatter {
            scattered,
            attenuation,
        })
    }
}
