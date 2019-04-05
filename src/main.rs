mod camera;
mod hitable;
mod pixbuf;
mod material;
mod ray;
mod vec3;

use camera::Camera;
use hitable::{Hitable, HitableList, Sphere};
use material::{Dielectric, Lambertian, Metal, Scatter};
use pixbuf::{Pixbuf};
use rand::prelude::*;
use ray::Ray;
use rayon::prelude::*;
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

fn render_once(nx:usize, ny:usize) -> Pixbuf {
    let depth = 50;
    let cam = Camera::new();
    let world = world();

    let mut res = Pixbuf::new(nx, ny);
    let mut rng = thread_rng();

    for j in 0..ny {
        for i in 0..nx {
            let u = (i as Float + rng.gen::<Float>()) / (nx as Float);
            let v = 1. - (j as Float + rng.gen::<Float>()) / (ny as Float);
            let r = cam.get_ray(u, v);
            let c = colour(&r, &world, depth);
            res.put(i, j, c);
        }
    }

    res
}

fn render() -> Pixbuf {
    let nx = 200;
    let ny = 100;
    let ns = 100;

    let mut res = (0..ns).into_par_iter()
        .map(|_| render_once(nx, ny))
        .reduce(|| Pixbuf::new(nx, ny), |mut i1, i2| {
            i1 += i2;
            i1
        });

    res /= ns;

    res
}

fn main() {
    render().as_image().save("image.png").unwrap();
}
