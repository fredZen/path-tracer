pub mod book_1;
pub mod book_2;
mod prelude;

use prelude::*;

fn camera(
    look_from: Pos,
    look_at: Pos,
    up: Dir,
    vfov: Float,
    aperture: Float,
    focus_dist: Float,
    settings: &Settings,
    t0: Float,
    t1: Float,
) -> Camera {
    let aspect = settings.width as Float / (settings.height as Float);
    Camera::new(
        look_from, look_at, up, vfov, aspect, aperture, focus_dist, t0, t1,
    )
}

pub struct Scene<C> {
    pub camera: Camera,
    pub world: HitableBox<C>,
}
