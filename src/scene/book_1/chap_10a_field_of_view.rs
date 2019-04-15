use crate::scene::prelude::*;

fn camera( settings: &Settings) -> Camera {
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

fn world<C>(factory: &HitableFactory<C>) -> HitableBox<C> {
    let r = (PI / 4.).cos();
    factory.hitable_list(vec![
        factory.sphere(pos(-r, 0., -1.), r, lambertian(col(0., 0., 1.))),
        factory.sphere(pos(r, 0., -1.), r, lambertian(col(1., 0., 0.))),
    ])
}

pub fn scene<C>(factory: &HitableFactory<C>, settings: &Settings) -> Scene<C> {
    Scene {
        camera: camera(settings),
        world: world(factory),
    }
}
