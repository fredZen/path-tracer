use crate::scene::prelude::*;

fn camera(settings: &Settings) -> Camera {
    let look_from = pos(3., 3., 2.);
    let look_at = pos(0., 0., -1.);
    let focus_dist = (look_at - look_from).length();
    crate::scene::camera(
        look_from,
        look_at,
        dir(0., 1., 0.),
        20.,
        2.,
        focus_dist,
        settings,
        0.,
        0.,
    )
}

fn world<C>(factory: &HitableFactory<C>) -> HitableBox<C> {
    factory.hitable_list(vec![
        factory.sphere(pos(0., 0., -1.), 0.5, lambertian(col(0.8, 0.3, 0.3))),
        factory.sphere(pos(0., -100.5, -1.), 100., lambertian(col(0.8, 0.8, 0.))),
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
