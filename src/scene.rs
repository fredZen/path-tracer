use crate::camera::Camera;
use crate::hitable::{hitable_list, sphere, HitableBox};
use crate::material::{dielectric, lambertian, metal};
use crate::vec3::{Col, Dir, Float, Pos, Vector};
use std::f32::consts::*;

pub struct Scene {
    pub camera: Camera,
    pub world: HitableBox,
    pub width: usize,
    pub height: usize,
    pub samples: usize,
    pub depth: usize,
}

fn camera(aspect: Float) -> Camera {
    let look_from = Pos::new(3., 3., 2.);
    let look_at = Pos::new(0., 0., -1.);
    let focus_dist = (look_at - look_from).length();
    Camera::new(
        look_from,
        look_at,
        Dir::new(0., 1., 0.),
        20.,
        aspect,
        2.,
        focus_dist,
    )
}

fn small_world() -> HitableBox {
    let r = (PI / 4.).cos();
    hitable_list(vec![
        sphere(Pos::new(-r, 0., -1.), r, lambertian(Col::new(0., 0., 1.))),
        sphere(Pos::new(r, 0., -1.), r, lambertian(Col::new(1., 0., 0.))),
    ])
}

fn world() -> HitableBox {
    hitable_list(vec![
        sphere(
            Pos::new(0., 0., -1.),
            0.5,
            lambertian(Col::new(0.1, 0.2, 0.5)),
        ),
        sphere(
            Pos::new(0., -100.5, -1.),
            100.,
            lambertian(Col::new(0.8, 0.8, 0.)),
        ),
        sphere(
            Pos::new(1., 0., -1.),
            0.5,
            metal(Col::new(0.8, 0.6, 0.2), 0.),
        ),
        sphere(Pos::new(-1., 0., -1.), 0.5, dielectric(1.9)),
        sphere(Pos::new(-1., 0., -1.), -0.45, dielectric(1.9)),
    ])
}

pub fn scene() -> Scene {
    let width = 200;
    let height = 100;
    let samples = 100;
    let depth = 50;
    let camera = camera((width as Float) / (height as Float));
    let world = world();
    Scene {
        width,
        height,
        samples,
        depth,
        camera,
        world,
    }
}
