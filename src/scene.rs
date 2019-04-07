use crate::camera::Camera;
use crate::hitable::{hitable_list, sphere, HitableBox};
use crate::material::{dielectric, lambertian, metal};
use crate::prelude::*;
use std::f32::consts::*;

pub struct Scene {
    pub camera: Camera,
    pub world: HitableBox,
    pub width: usize,
    pub height: usize,
    pub samples: usize,
    pub depth: usize,
}

fn camera(aspect: Float) -> Camera {
    let look_from = pos(3., 3., 2.);
    let look_at = pos(0., 0., -1.);
    let focus_dist = (look_at - look_from).length();
    Camera::new(
        look_from,
        look_at,
        dir(0., 1., 0.),
        20.,
        aspect,
        2.,
        focus_dist,
    )
}

fn small_world() -> HitableBox {
    let r = (PI / 4.).cos();
    hitable_list(vec![
        sphere(pos(-r, 0., -1.), r, lambertian(col(0., 0., 1.))),
        sphere(pos(r, 0., -1.), r, lambertian(col(1., 0., 0.))),
    ])
}

fn world() -> HitableBox {
    hitable_list(vec![
        sphere(pos(0., 0., -1.), 0.5, lambertian(col(0.1, 0.2, 0.5))),
        sphere(pos(0., -100.5, -1.), 100., lambertian(col(0.8, 0.8, 0.))),
        sphere(pos(1., 0., -1.), 0.5, metal(col(0.8, 0.6, 0.2), 0.)),
        sphere(pos(-1., 0., -1.), 0.5, dielectric(1.9)),
        sphere(pos(-1., 0., -1.), -0.45, dielectric(1.9)),
    ])
}

pub fn scene() -> Scene {
    let width = 200;
    let height = 100;
    let samples = 100;
    let depth = 50;
    let camera = camera((width as Float) / (height as Float));
    let world = world();
    Scene {
        width,
        height,
        samples,
        depth,
        camera,
        world,
    }
}
