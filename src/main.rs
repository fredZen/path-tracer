mod camera;
mod hitable;
mod material;
mod pixbuf;
mod prelude;
mod ray;
mod scene;
mod settings;
mod texture;
mod vec3;

use hitable::{HitableFactory, PlainHitableFactory, Stats, TracingHitableFactory};
use pixbuf::Pixbuf;
use prelude::*;
use rand::prelude::*;
use rayon::prelude::*;
use scene::Scene;

pub struct Settings {
    pub width: usize,
    pub height: usize,
    pub samples: usize,
    pub depth: usize,
}

#[inline]
fn colour<C>(c: &mut C, r: &Ray, world: &Hitable<C>, depth: usize) -> Col {
    if let Some(rec) = world.hit(c, r, 0.001, std::f32::MAX) {
        if depth > 0 {
            if let Some(Scatter {
                scattered,
                attenuation,
            }) = rec.mat.scatter(r, &rec)
            {
                return attenuation * colour(c, &scattered, world, depth - 1);
            }
        }

        return Col::zero();
    } else {
        let unit_direction = r.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.);
        (1. - t) * col(1., 1., 1.) + t * col(0.5, 0.7, 1.0)
    }
}

fn render_once<C>(c: &mut C, settings: &Settings, scene: &Scene<C>) -> Pixbuf {
    let &Settings {
        width,
        height,
        depth,
        ..
    } = settings;
    let Scene { world, camera } = scene;
    let mut res = Pixbuf::new(width, height);
    let mut rng = thread_rng();

    for j in 0..height {
        for i in 0..width {
            let u = (i as Float + rng.gen::<Float>()) / (width as Float);
            let v = 1. - (j as Float + rng.gen::<Float>()) / (height as Float);
            let r = camera.get_ray(u, v);
            let c = colour(c, &r, &**world, depth);
            res.put(i, j, c);
        }
    }

    res
}

fn render<F>(settings: Settings, scene: F) -> Pixbuf
where
    F: Fn(&PlainHitableFactory, &Settings) -> Scene<()>,
{
    let scene = scene(&PlainHitableFactory, &settings);
    let mut pixbuf = (0..settings.samples)
        .into_par_iter()
        .map(|_| {
            let pixbuf = render_once(&mut (), &settings, &scene);
            pixbuf
        })
        .reduce(
            || (Pixbuf::new(settings.width, settings.height)),
            |mut p1, p2| {
                p1 += p2;
                p1
            },
        );

    pixbuf /= settings.samples;

    pixbuf
}

fn render_with_stats<F>(settings: Settings, scene: F) -> Pixbuf
where
    F: Fn(&TracingHitableFactory, &Settings) -> Scene<Stats>,
{
    let scene = scene(&TracingHitableFactory, &settings);
    let (stats, mut pixbuf) = (0..settings.samples)
        .into_par_iter()
        .map(|_| {
            let mut stats = Stats::new();
            let pixbuf = render_once(&mut stats, &settings, &scene);
            (stats, pixbuf)
        })
        .reduce(
            || (Stats::new(), Pixbuf::new(settings.width, settings.height)),
            |(mut s1, mut p1), (s2, p2)| {
                s1 += s2;
                p1 += p2;
                (s1, p1)
            },
        );

    println!("{:#?}", stats);
    pixbuf /= settings.samples;

    pixbuf
}

fn main() {
    let settings = settings::high();

    // let scene = scene::book_1::chap_03_simple_camera_and_background::scene;
    // let scene = scene::book_1::chap_07_sphere::scene;
    // let scene = scene::book_1::chap_08a_lambertian::scene;
    // let scene = scene::book_1::chap_08b_metal::scene;
    // let scene = scene::book_1::chap_08c_fuzzy_metal::scene;
    // let scene = scene::book_1::chap_09a_dielectric::scene;
    // let scene = scene::book_1::chap_09b_hollow_dielectric::scene;
    // let scene = scene::book_1::chap_10a_field_of_view::scene;
    // let scene = scene::book_1::chap_10b_positionable_camera::scene;
    // let scene = scene::book_1::chap_11_depth_of_field::scene;
    // let scene = scene::book_1::chap_12_book_cover::scene;
    // let scene = scene::book_2::chap_01_motion_blur::scene;
    // let scene = scene::book_2::chap_02_bounding_volumes::scene;
    // let scene = scene::book_2::chap_03a_checker_floor::scene;
    let scene = scene::book_2::chap_03b_checker_spheres::scene;

    render(settings, scene)
        .as_image()
        .save("image.png")
        .unwrap();
}
