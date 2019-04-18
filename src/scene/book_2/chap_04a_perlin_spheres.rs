use crate::scene::prelude::*;

fn camera(settings: &Settings) -> Camera {
    let look_from = pos(13., 2., 3.);
    let look_at = pos(0., 0., 0.);
    crate::scene::camera(
        look_from,
        look_at,
        dir(0., 1., 0.),
        20.,
        0.,
        10.,
        settings,
        0.,
        0.,
    )
}

fn world<C>(factory: &HitableFactory<C>) -> HitableBox<C> {
    let mut list = vec![];
    list.push(factory.sphere(pos(0., -1000., 0.), 1000., lambertian(noise_texture())));
    list.push(factory.sphere(pos(0., 2., 0.), 2., lambertian(noise_texture())));
    factory.bounding_hierarchy(list, 0., 0.)
}

pub fn scene<C, H: HitableFactory<C>>(factory: &H, settings: &Settings) -> Scene<C> {
    Scene {
        camera: camera(settings),
        world: world(factory),
    }
}
