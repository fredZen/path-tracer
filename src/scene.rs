pub mod chap_03_simple_camera_and_background;
pub mod chap_07_sphere;
pub mod chap_08a_lambertian;
pub mod chap_08b_metal;
pub mod chap_08c_fuzzy_metal;
pub mod chap_09a_dielectric;
pub mod chap_09b_hollow_dielectric;
pub mod chap_10a_field_of_view;
pub mod chap_10b_positionable_camera;
pub mod chap_11_depth_of_field;
pub mod chap_12_book_cover;

use super::Settings;
use crate::camera::Camera;
use crate::hitable::HitableBox;
use crate::prelude::*;

fn camera(
    look_from: Pos,
    look_at: Pos,
    up: Dir,
    vfov: Float,
    aperture: Float,
    focus_dist: Float,
    settings: &Settings,
) -> Camera {
    let aspect = settings.width as Float / (settings.height as Float);
    Camera::new(look_from, look_at, up, vfov, aspect, aperture, focus_dist)
}

pub struct Scene {
    pub camera: Camera,
    pub world: HitableBox,
}
