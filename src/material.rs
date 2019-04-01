mod lambertian;

pub use lambertian::Lambertian;

use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Col;

pub struct Scatter {
    pub attenuation: Col,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter>;
}
