pub use crate::{
    hitable::{HitRecord, Hitable},
    material::{Material, Scatter},
    ray::Ray,
    texture::{Texture, TextureBox},
    vec3::{
        col::{col, Col},
        dir::{dir, Dir},
        pos::{pos, Pos},
        Float, Vector,
    },
};
pub use std::f32::{consts::*, MAX};
