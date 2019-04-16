use crate::scene::prelude::*;

fn camera(settings: &Settings) -> Camera {
    let look_from = pos(13., 2., 3.);
    let look_at = pos(0., 0., 0.);
    crate::scene::camera(
        look_from,
        look_at,
        dir(0., 1., 0.),
        20.,
        0.1,
        10.,
        settings,
        0.,
        1.,
    )
}

fn world<C>(factory: &HitableFactory<C>) -> HitableBox<C> {
    let mut rng = thread_rng();
    let mut list = vec![];
    list.push(factory.sphere(pos(0., -1000., 0.), 1000., lambertian(col(0.5, 0.5, 0.5))));

    fn random_sphere<C>(factory: &HitableFactory<C>, center: Pos) -> HitableBox<C> {
        let mut rng = thread_rng();
        let choose_mat = rng.gen::<Float>();
        if choose_mat < 0.8 {
            factory.moving_sphere(
                center,
                center + dir(0., 0.5 * rng.gen::<Float>(), 0.),
                0.,
                1.,
                0.2,
                lambertian(col(
                    rng.gen::<Float>() * rng.gen::<Float>(),
                    rng.gen::<Float>() * rng.gen::<Float>(),
                    rng.gen::<Float>() * rng.gen::<Float>(),
                )),
            )
        } else if choose_mat < 0.95 {
            factory.sphere(
                center,
                0.2,
                metal(
                    col(
                        0.5 * (1. + rng.gen::<Float>()),
                        0.5 * (1. + rng.gen::<Float>()),
                        0.5 * (1. + rng.gen::<Float>()),
                    ),
                    0.5 * rng.gen::<Float>(),
                ),
            )
        } else {
            factory.sphere(center, 0.2, dielectric(1.5))
        }
    }

    for a in -11..11 {
        for b in -11..11 {
            let center = pos(
                a as Float + 0.9 * rng.gen::<Float>(),
                0.2,
                b as Float + 0.9 * rng.gen::<Float>(),
            );
            if (center - pos(4., 0.2, 0.)).length() > 0.9 {
                list.push(random_sphere(factory, center))
            }
        }
    }

    list.push(factory.sphere(pos(0., 1., 0.), 1., dielectric(1.5)));
    list.push(factory.sphere(pos(-4., 1., 0.), 1., lambertian(col(0.4, 0.2, 0.1))));
    list.push(factory.sphere(pos(4., 1., 0.), 1., metal(col(0.7, 0.6, 0.5), 0.)));
    factory.hitable_list(list)
}

pub fn scene<C, H: HitableFactory<C>>(factory: &H, settings: &Settings) -> Scene<C> {
    Scene {
        camera: camera(settings),
        world: world(factory),
    }
}
