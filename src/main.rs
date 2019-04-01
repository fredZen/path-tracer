mod camera;
mod hitable;
mod ray;
mod vec3;

use camera::Camera;
use hitable::{HitRecord, Hitable, HitableList, Sphere};
use ray::Ray;
use vec3::{Col, Float, Pos, Vector};
use rand::prelude::*;

fn colour(r: &Ray, hitable: &Hitable) -> Col {
    if let Some(HitRecord { normal, .. }) = hitable.hit(r, 0., std::f32::MAX) {
        0.5 * Col::new(normal.x() + 1., normal.y() + 1., normal.z() + 1.)
    } else {
        let unit_direction = r.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.);
        (1. - t) * Col::new(1., 1., 1.) + t * Col::new(0.5, 0.7, 1.0)
    }
}

fn world() -> HitableList {
    HitableList::new(vec![
        Box::new(Sphere::new(Pos::new(0., 0., -1.), 0.5)),
        Box::new(Sphere::new(Pos::new(0., -100.5, -1.), 100.)),
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
                let r = cam.get_ray(u,v);
                col += colour(&r, &world);
            }
            col /= Float::from(ns);

            let ir = as_u8(col.r());
            let ig = as_u8(col.g());
            let ib = as_u8(col.b());

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
