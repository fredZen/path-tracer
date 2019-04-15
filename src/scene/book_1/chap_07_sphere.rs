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
    factory.hitable_list(vec![
        factory.sphere(pos(0., 0., -1.), 0.5, lambertian(col(0.5, 0.5, 0.5))),
        factory.sphere(pos(0., -100.5, -1.), 100., lambertian(col(0.5, 0.5, 0.5))),
    ])
}

pub fn scene<C: 'static>(factory: &HitableFactory<C>, settings: &Settings) -> Scene<C> {
    Scene {
        camera: camera(settings),
        world: world(factory),
    }
}
