mod camera;
mod hitable;
mod material;
mod pixbuf;
mod prelude;
mod ray;
mod scene;
mod settings;
mod vec3;

use pixbuf::Pixbuf;
use prelude::*;
use rand::prelude::*;
use rayon::prelude::*;

pub struct Settings {
    pub width: usize,
    pub height: usize,
    pub samples: usize,
    pub depth: usize,
}

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
        (1. - t) * col(1., 1., 1.) + t * col(0.5, 0.7, 1.0)
    }
}

fn render_once(settings: &Settings, scene: &scene::Scene) -> Pixbuf {
    let &Settings {
        width,
        height,
        depth,
        ..
    } = settings;
    let scene::Scene { world, camera } = scene;
    let mut res = Pixbuf::new(width, height);
    let mut rng = thread_rng();

    for j in 0..height {
        for i in 0..width {
            let u = (i as Float + rng.gen::<Float>()) / (width as Float);
            let v = 1. - (j as Float + rng.gen::<Float>()) / (height as Float);
            let r = camera.get_ray(u, v);
            let c = colour(&r, &**world, depth);
            res.put(i, j, c);
        }
    }

    res
}

fn render() -> Pixbuf {
    let settings = settings::low();
    let scene = scene::chap_12_book_cover::scene(&settings);

    let mut res = (0..settings.samples)
        .into_par_iter()
        .map(|_| render_once(&settings, &scene))
        .reduce(
            || Pixbuf::new(settings.width, settings.height),
            |mut i1, i2| {
                i1 += i2;
                i1
            },
        );

    res /= settings.samples;

    res
}

fn main() {
    render().as_image().save("image.png").unwrap();
}
