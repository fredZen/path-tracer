use crate::scene::prelude::*;

fn camera(settings: &Settings, t0: Float, t1: Float) -> Camera {
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
        t0,
        t1,
    )
}

fn world(t0: Float, t1: Float) -> HitableBox {
    let mut rng = thread_rng();
    let mut list = vec![];
    list.push(sphere(
        pos(0., -1000., 0.),
        1000.,
        lambertian(col(0.5, 0.5, 0.5)),
    ));

    fn random_sphere(center: Pos) -> HitableBox {
        let mut rng = thread_rng();
        let choose_mat = rng.gen::<Float>();
        if choose_mat < 0.8 {
            moving_sphere(
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
            sphere(
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
            sphere(center, 0.2, dielectric(1.5))
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
                list.push(random_sphere(center))
            }
        }
    }

    list.push(sphere(pos(0., 1., 0.), 1., dielectric(1.5)));
    list.push(sphere(pos(-4., 1., 0.), 1., lambertian(col(0.4, 0.2, 0.1))));
    list.push(sphere(pos(4., 1., 0.), 1., metal(col(0.7, 0.6, 0.5), 0.)));
    bounding_hierarchy(list, t0, t1)
}

pub fn scene(settings: &Settings) -> Scene {
    let t0 = 0.;
    let t1 = 1.;
    Scene {
        camera: camera(settings, t0, t1),
        world: world(t0, t1),
    }
}
