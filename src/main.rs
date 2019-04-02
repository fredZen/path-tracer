mod camera;
mod hitable;
mod material;
mod ray;
mod vec3;

use camera::Camera;
use hitable::{Hitable, HitableList, Sphere};
use material::{Dielectric, Lambertian, Metal, Scatter};
use rand::prelude::*;
use ray::Ray;
use vec3::{Col, Float, Pos, Vector};

fn colour(r: &Ray, world: &Hitable, depth: u8) -> Col {
    if let Some(rec) = world.hit(r, 0.001, std::f32::MAX) {
        if depth > 0 {
            if let Some(Scatter {
                scattered,
                attenuation,
            }) = rec.mat.scatter(r, &rec)
            {
                return attenuation * colour(&scattered, world, depth - 1);
            }
        }

        return Col::zero();
    } else {
        let unit_direction = r.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.);
        (1. - t) * Col::new(1., 1., 1.) + t * Col::new(0.5, 0.7, 1.0)
    }
}

fn world() -> HitableList {
    HitableList::new(vec![
        Box::new(Sphere::new(
            Pos::new(0., 0., -1.),
            0.5,
            Box::new(Lambertian::new(Col::new(0.1, 0.2, 0.5))),
        )),
        Box::new(Sphere::new(
            Pos::new(0., -100.5, -1.),
            100.,
            Box::new(Lambertian::new(Col::new(0.8, 0.8, 0.))),
        )),
        Box::new(Sphere::new(
            Pos::new(1., 0., -1.),
            0.5,
            Box::new(Metal::new(Col::new(0.8, 0.6, 0.2), 0.)),
        )),
        Box::new(Sphere::new(
            Pos::new(-1., 0., -1.),
            0.5,
            Box::new(Dielectric::new(1.9)),
        )),
        Box::new(Sphere::new(
            Pos::new(-1., 0., -1.),
            -0.45,
            Box::new(Dielectric::new(1.9)),
        )),
    ])
}

fn as_u8(f: Float) -> u8 {
    if f < 1. {
        (256. * f) as u8
    } else {
        255
    }
}

fn main() {
    let nx = 200u16;
    let ny = 100u16;
    let ns = 100u16;
    let depth = 50;
    let mut rng = thread_rng();
    let cam = Camera::new();
    let world = world();

    print!("P3\n{} {}\n255\n", nx, ny);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Col::zero();
            for _ in 0..ns {
                let u = (Float::from(i) + rng.gen::<Float>()) / Float::from(nx);
                let v = (Float::from(j) + rng.gen::<Float>()) / Float::from(ny);
                let r = cam.get_ray(u, v);
                col += colour(&r, &world, depth);
            }
            col /= Float::from(ns);
            let col = Col::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());

            let ir = as_u8(col.r());
            let ig = as_u8(col.g());
            let ib = as_u8(col.b());

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
