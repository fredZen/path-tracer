use super::Scene;
use crate::camera::Camera;
use crate::hitable::{hitable_list, HitableBox};
use crate::prelude::*;
use crate::Settings;

fn camera(settings: &Settings) -> Camera {
    let look_from = pos(0., 0., 0.);
    let look_at = pos(0., 0., -1.);
    super::camera(look_from, look_at, dir(0., 1., 0.), 90., 0., 1., settings)
}

fn world() -> HitableBox {
    hitable_list(vec![])
}

pub fn scene(settings: &Settings) -> Scene {
    Scene {
        camera: camera(settings),
        world: world(),
    }
}
