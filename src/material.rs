mod dielectric;
mod lambertian;
mod metal;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;

use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Col, Dir, Vector};
use rand::prelude::*;

pub struct Scatter {
    pub attenuation: Col,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter>;
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

fn reflect(v: Dir, n: Dir) -> Dir {
    v - 2. * v.dot(n) * n
}
