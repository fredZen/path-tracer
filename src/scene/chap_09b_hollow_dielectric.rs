use super::Scene;
use crate::camera::Camera;
use crate::hitable::{hitable_list, sphere, HitableBox};
use crate::material::{dielectric, lambertian, metal};
use crate::prelude::*;
use crate::Settings;

fn camera(settings: &Settings) -> Camera {
    let look_from = pos(0., 0., 0.);
    let look_at = pos(0., 0., -1.);
    super::camera(look_from, look_at, dir(0., 1., 0.), 90., 0., 1., settings)
}

fn world() -> HitableBox {
    hitable_list(vec![
        sphere(pos(0., 0., -1.), 0.5, lambertian(col(0.8, 0.3, 0.3))),
        sphere(pos(0., -100.5, -1.), 100., lambertian(col(0.8, 0.8, 0.))),
        sphere(pos(1., 0., -1.), 0.5, metal(col(0.8, 0.6, 0.2), 0.)),
        sphere(pos(-1., 0., -1.), 0.5, dielectric(1.9)),
        sphere(pos(-1., 0., -1.), -0.45, dielectric(1.9)),
    ])
}

pub fn scene(settings: &Settings) -> Scene {
    Scene {
        camera: camera(settings),
        world: world(),
    }
}
