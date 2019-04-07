mod dielectric;
mod lambertian;
mod metal;

use crate::hitable::HitRecord;
use crate::prelude::*;
use crate::ray::Ray;
use rand::prelude::*;

pub struct Scatter {
    pub attenuation: Col,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter>;
}

pub type MaterialBox = Box<Material + Send + Sync>;

pub fn dielectric(ri: Float) -> MaterialBox {
    Box::new(dielectric::Dielectric::new(ri))
}

pub fn lambertian(albedo: Col) -> MaterialBox {
    Box::new(lambertian::Lambertian::new(albedo))
}

pub fn metal(albedo: Col, fuzz: Float) -> MaterialBox {
    Box::new(metal::Metal::new(albedo, fuzz))
}

fn random_in_unit_sphere() -> Dir {
    let mut rng = thread_rng();
    loop {
        let p = 2. * dir(rng.gen(), rng.gen(), rng.gen()) - dir(1., 1., 1.);
        if p.squared_length() < 1. {
            return p;
        }
    }
}

fn reflect(v: Dir, n: Dir) -> Dir {
    v - 2. * v.dot(n) * n
}
