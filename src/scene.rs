use crate::camera::Camera;
use crate::hitable::{Hitable, HitableList, Sphere};
use crate::material::{Dielectric, Lambertian, Metal};
use crate::vec3::{Col, Pos};

pub struct Scene<H: Hitable + Send + Sync> {
    pub camera: Camera,
    pub world: H,
    pub width: usize,
    pub height: usize,
    pub samples: usize,
    pub depth: usize,
}

fn camera() -> Camera {
    Camera::new()
}

#[inline]
fn world() -> HitableList {
    HitableList::new(vec![
        Box::new(Sphere::new(
            Pos::new(0., 0., -1.),
            0.5,
            Box::new(Lambertian::new(Col::new(0.1, 0.2, 0.5))),
        )),
        Box::new(Sphere::new(
            Pos::new(0., -100.5, -1.),
            100.,
            Box::new(Lambertian::new(Col::new(0.8, 0.8, 0.))),
        )),
        Box::new(Sphere::new(
            Pos::new(1., 0., -1.),
            0.5,
            Box::new(Metal::new(Col::new(0.8, 0.6, 0.2), 0.)),
        )),
        Box::new(Sphere::new(
            Pos::new(-1., 0., -1.),
            0.5,
            Box::new(Dielectric::new(1.9)),
        )),
        Box::new(Sphere::new(
            Pos::new(-1., 0., -1.),
            -0.45,
            Box::new(Dielectric::new(1.9)),
        )),
    ])
}

pub fn scene() -> Scene<HitableList> {
    let width = 200;
    let height = 100;
    let samples = 100;
    let depth = 50;
    let camera = camera();
    let world = world();
    Scene{ width, height, samples, depth, camera, world }
}
