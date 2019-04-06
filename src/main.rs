mod camera;
mod hitable;
mod pixbuf;
mod material;
mod ray;
mod scene;
mod vec3;

use hitable::{Hitable};
use material::Scatter;
use pixbuf::{Pixbuf};
use rand::prelude::*;
use ray::Ray;
use rayon::prelude::*;
use vec3::{Col, Float, Vector};

#[inline]
fn colour(r: &Ray, world: &Hitable, depth: usize) -> Col {
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

fn render_once<T: Hitable + Send + Sync>(scene: &scene::Scene<T>) -> Pixbuf {
    let &scene::Scene{width, height, depth, ..} = scene;
    let scene::Scene{world, camera, ..} = scene;
    let mut res = Pixbuf::new(width, height);
    let mut rng = thread_rng();

    for j in 0..height {
        for i in 0..width {
            let u = (i as Float + rng.gen::<Float>()) / (width as Float);
            let v = 1. - (j as Float + rng.gen::<Float>()) / (height as Float);
            let r = camera.get_ray(u, v);
            let c = colour(&r, world, depth);
            res.put(i, j, c);
        }
    }

    res
}

fn render() -> Pixbuf {
    let scene = scene::scene();
    let scene::Scene{samples, width, height, ..} = scene;

    let mut res = (0..samples).into_par_iter()
        .map(|_| render_once(&scene))
        .reduce(|| Pixbuf::new(width, height), |mut i1, i2| {
            i1 += i2;
            i1
        });

    res /= samples;

    res
}

fn main() {
    render().as_image().save("image.png").unwrap();
}
