use crate::camera::Camera;
use crate::hitable::{hitable_list, sphere, HitableBox};
use crate::material::{dielectric, lambertian, metal};
use crate::prelude::*;
use crate::scene::Scene;
use crate::Settings;

fn camera(settings: &Settings) -> Camera {
    let look_from = pos(0., 0., 0.);
    let look_at = pos(0., 0., -1.);
    crate::scene::camera(
        look_from,
        look_at,
        dir(0., 1., 0.),
        90.,
        0.,
        1.,
        settings,
        0.,
        0.,
    )
}

fn world() -> HitableBox {
    let r = (PI / 4.).cos();
    hitable_list(vec![
        sphere(pos(-r, 0., -1.), r, lambertian(col(0., 0., 1.))),
        sphere(pos(r, 0., -1.), r, lambertian(col(1., 0., 0.))),
    ])
}

pub fn scene(settings: &Settings) -> Scene {
    Scene {
        camera: camera(settings),
        world: world(),
    }
}
