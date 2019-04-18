use crate::prelude::*;
use lazy_static::lazy_static;
use rand::{distributions::Standard, prelude::*};

fn generate_perm() -> Vec<usize> {
    let mut res: Vec<usize> = (0..=255).collect();
    res.shuffle(&mut thread_rng());
    res
}

struct PerlinData {
    ranfloat: Vec<Float>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}
lazy_static! {
    static ref PERLIN_DATA: PerlinData = PerlinData {
        ranfloat: thread_rng().sample_iter(&Standard).take(256).collect(),
        perm_x: generate_perm(),
        perm_y: generate_perm(),
        perm_z: generate_perm(),
    };
}

pub fn noise(p: Pos) -> Float {
    let u = p.x() - p.x().floor();
    let v = p.y() - p.y().floor();
    let w = p.z() - p.z().floor();

    let i = ((4. * p.x()) as i64 & 255) as usize;
    let j = ((4. * p.y()) as i64 & 255) as usize;
    let k = ((4. * p.z()) as i64 & 255) as usize;

    let PerlinData{ranfloat, perm_x, perm_y, perm_z} = &*PERLIN_DATA;

    ranfloat[perm_x[i] ^ perm_y[j] ^ perm_z[k]]
}

#[derive(Clone, Debug)]
pub struct NoiseTexture;

impl Texture for NoiseTexture {
    fn value(&self, u: Float, v: Float, p: Pos) -> Col {
        col(1., 1., 1.) * noise(p)
    }
}
