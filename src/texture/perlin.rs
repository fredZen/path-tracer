use crate::prelude::*;
use lazy_static::lazy_static;
use rand::prelude::*;

fn generate_perm() -> Vec<usize> {
    let mut res: Vec<usize> = (0..=255).collect();
    res.shuffle(&mut thread_rng());
    res
}

struct PerlinData {
    ranvec: Vec<Dir>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

lazy_static! {
    static ref PERLIN_DATA: PerlinData = PerlinData {
        ranvec: {
            let mut rng = thread_rng();
            (0..256)
                .map(|_| {
                    dir(
                        -1. + 2. * rng.gen::<Float>(),
                        -1. + 2. * rng.gen::<Float>(),
                        -1. + 2. * rng.gen::<Float>(),
                    )
                    .unit_vector()
                })
                .collect()
        },
        perm_x: generate_perm(),
        perm_y: generate_perm(),
        perm_z: generate_perm(),
    };
}

#[inline]
pub fn perlin_interp(c: [[[Dir; 2]; 2]; 2], u: Float, v: Float, w: Float) -> Float {
    let uu = u * u * (3. - 2. * u);
    let vv = v * v * (3. - 2. * v);
    let ww = w * w * (3. - 2. * w);

    let mut accum = 0.;


    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let c = c[i][j][k];

                let (i, j, k) = (i as Float, j as Float, k as Float);
                let weight = dir(u - i, v - j, w - k);

                let mut c = c.dot(weight);
                c *= i * uu + (1. - i) * (1. - uu);
                c *= j * vv + (1. - j) * (1. - vv);
                c *= k * ww + (1. - k) * (1. - ww);

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

    let mut c = [[[Dir::zero(); 2]; 2]; 2];

    let PerlinData {
        ranvec,
        perm_x,
        perm_y,
        perm_z,
    } = &*PERLIN_DATA;

    for di in 0..2 {
        for dj in 0..2 {
            for dk in 0..2 {
                let px = perm_x[(i as usize + di) & 255];
                let py = perm_y[(j as usize + dj) & 255];
                let pz = perm_z[(k as usize + dk) & 255];

                c[di][dj][dk] = ranvec[px ^ py ^ pz];
            }
        }
    }

    perlin_interp(c, u, v, w)
}

#[derive(Clone, Debug)]
pub struct NoiseTexture {
    scale: Float,
}

impl NoiseTexture {
    pub fn new(scale: Float) -> Self {
        NoiseTexture { scale }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: Float, _v: Float, p: Pos) -> Col {
        col(1., 1., 1.) * 0.5 * (1. + noise(self.scale * p))
    }
}
