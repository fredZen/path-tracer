mod hitable;
mod ray;
mod vec3;

use hitable::{HitRecord, Hitable, HitableList, Sphere};
use ray::Ray;
use vec3::{Col, Dir, Float, Pos, Vector};

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
    let world = world();

    print!("P3\n{} {}\n255\n", nx, ny);
    let lower_left_corner = Dir::new(-2., -1., -1.);
    let horizontal = Dir::new(4., 0., 0.);
    let vertical = Dir::new(0., 2., 0.);
    let origin = Pos::new(0., 0., 0.);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = Float::from(i) / Float::from(nx);
            let v = Float::from(j) / Float::from(ny);
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let col = colour(&r, &world);

            let ir = as_u8(col.r());
            let ig = as_u8(col.g());
            let ib = as_u8(col.b());

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
