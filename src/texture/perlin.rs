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

#[inline]
pub fn trilinear_interp(c: [[[Float; 2]; 2]; 2], u: Float, v: Float, w: Float) -> Float {
    let mut accum = 0.;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let mut c = c[i][j][k];

                let i = i as Float;
                let j = j as Float;
                let k = k as Float;

                c *= i * u + (1. - i) * (1. - u);
                c *= j * v + (1. - j) * (1. - v);
                c *= k * w + (1. - k) * (1. - w);

                accum += c;
            }
        }
    }

    accum
}

pub fn noise(p: Pos) -> Float {
    let i = p.x().floor();
    let j = p.y().floor();
    let k = p.z().floor();

    let u = p.x() - i;
    let v = p.y() - j;
    let w = p.z() - k;

    let u = u * u * (3. - 2. * u);
    let v = v * v * (3. - 2. * v);
    let w = w * w * (3. - 2. * w);

    let i = i as usize;
    let j = j as usize;
    let k = k as usize;

    let mut c = [[[0 as Float; 2]; 2]; 2];

    let PerlinData {
        ranfloat,
        perm_x,
        perm_y,
        perm_z,
    } = &*PERLIN_DATA;

    for di in 0..2 {
        for dj in 0..2 {
            for dk in 0..2 {
                let px = perm_x[(i + di) & 255];
                let py = perm_y[(j + dj) & 255];
                let pz = perm_z[(k + dk) & 255];

                c[di][dj][dk] = ranfloat[px ^ py ^ pz];
            }
        }
    }

    trilinear_interp(c, u, v, w)
}

#[derive(Clone, Debug)]
pub struct NoiseTexture;

impl Texture for NoiseTexture {
    fn value(&self, _u: Float, _v: Float, p: Pos) -> Col {
        col(1., 1., 1.) * noise(p)
    }
}
