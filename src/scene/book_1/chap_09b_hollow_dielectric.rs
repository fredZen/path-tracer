use crate::scene::prelude::*;

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

fn world<C>(factory: &HitableFactory<C>) -> HitableBox<C> {
    factory.hitable_list(vec![
        factory.sphere(
            pos(0., 0., -1.),
            0.5,
            lambertian(constant_texture(col(0.8, 0.3, 0.3))),
        ),
        factory.sphere(
            pos(0., -100.5, -1.),
            100.,
            lambertian(constant_texture(col(0.8, 0.8, 0.))),
        ),
        factory.sphere(pos(1., 0., -1.), 0.5, metal(col(0.8, 0.6, 0.2), 0.)),
        factory.sphere(pos(-1., 0., -1.), 0.5, dielectric(1.9)),
        factory.sphere(pos(-1., 0., -1.), -0.45, dielectric(1.9)),
    ])
}

pub fn scene<C, H: HitableFactory<C>>(factory: &H, settings: &Settings) -> Scene<C> {
    Scene {
        camera: camera(settings),
        world: world(factory),
    }
}
