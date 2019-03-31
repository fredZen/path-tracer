mod ray;
mod vec3;

use ray::Ray;
use vec3::{Col, Dir, Float, Pos, Vector};

fn hit_sphere(center: Pos, radius: Float, r: &Ray) -> Option<Float> {
    let oc = r.origin() - center;
    let a = r.direction().dot(r.direction());
    let b = 2. * oc.dot(r.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b*b - 4. * a * c;
    if discriminant > 0. {
        Some(-b - (2. * a).sqrt())
    } else {
        None
    }
}

fn colour(r: &Ray) -> Col {
    if let Some(t) = hit_sphere(Pos::new(0., 0., -1.), 0.5, r) {
        let n = (r.point_at(t) - Pos::new(0., 0., -1.)).unit_vector();
        0.5 * Col::new(n.x() + 1., n.y() + 1., n.z() + 1.)
    } else {
        let unit_direction = r.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.);
        (1. - t) * Col::new(1., 1., 1.) + t * Col::new(0.5, 0.7, 1.0)
    }
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
            let col = colour(&r);

            let ir = as_u8(col.r());
            let ig = as_u8(col.g());
            let ib = as_u8(col.b());

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
