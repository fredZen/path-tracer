use super::{Material, Scatter, random_in_unit_sphere, reflect};
use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Col, Vector, Float};

pub struct Metal {
    albedo: Col,
    fuzz: Float,
}

impl Metal {
    pub fn new(albedo: Col, fuzz: Float) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, &HitRecord { p, normal, .. }: &HitRecord) -> Option<Scatter> {
        let reflected = reflect(r_in.direction().unit_vector(), normal);

        let scattered = Ray::new(p, reflected + self.fuzz * random_in_unit_sphere());
        let attenuation = self.albedo;

        if scattered.direction().dot(normal) > 0. {
            Some(Scatter {
                scattered,
                attenuation,
            })
        } else {
            None
        }
    }
}
