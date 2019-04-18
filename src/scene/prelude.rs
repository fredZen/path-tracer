pub use crate::{
    camera::Camera,
    hitable::{HitableBox, HitableFactory, Stats},
    material::{dielectric, lambertian, metal, MaterialBox},
    prelude::*,
    scene::Scene,
    texture::{checker, constant_texture, noise_texture},
    Settings,
};
pub use rand::prelude::*;
