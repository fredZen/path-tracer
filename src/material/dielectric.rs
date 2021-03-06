use super::reflect;
use crate::prelude::*;
use rand::prelude::*;

#[derive(Debug)]
pub struct Dielectric {
    ref_idx: Float,
}

fn refract(v: Dir, n: Dir, ni_over_nt: Float) -> Option<Dir> {
    let uv = v.unit_vector();
    let dt = uv.dot(n);
    let discriminant = 1. - ni_over_nt * ni_over_nt * (1. - dt * dt);
    if discriminant > 0. {
        Some(ni_over_nt * (uv - dt * n) - discriminant.sqrt() * n)
    } else {
        None
    }
}

fn schlick(cosine: Float, ref_idx: Float) -> Float {
    let r0 = (1. - ref_idx) / (1. + ref_idx);
    let r0 = r0 * r0;
    r0 + (1. - r0) * (1. - cosine).powi(5)
}

impl Dielectric {
    pub fn new(ri: Float) -> Dielectric {
        Dielectric { ref_idx: ri }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let attenuation = col(1., 1., 1.);
        let (outward_normal, ni_over_nt, cosine) = if r_in.direction().dot(rec.normal) > 0. {
            (
                -rec.normal,
                self.ref_idx,
                self.ref_idx * r_in.direction().dot(rec.normal) / r_in.direction().length(),
            )
        } else {
            (
                rec.normal,
                1. / self.ref_idx,
                -r_in.direction().dot(rec.normal) / r_in.direction().length(),
            )
        };
        if let Some(refracted) = refract(r_in.direction(), outward_normal, ni_over_nt) {
            let reflect_prob = schlick(cosine, self.ref_idx);

            if thread_rng().gen::<Float>() > reflect_prob {
                return Some(Scatter {
                    attenuation,
                    scattered: Ray::new(rec.p, refracted, r_in.time()),
                });
            }
        }
        let reflected = reflect(r_in.direction(), rec.normal);
        Some(Scatter {
            attenuation,
            scattered: Ray::new(rec.p, reflected, r_in.time()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_refract_no_change() {
        let v1 = dir(1., 1., 0.);
        let n = dir(0., -1., 0.);
        let v2 = v1.unit_vector();
        let refracted = refract(v1, n, 1.);
        assert!(refracted.is_some());

        assert_approx_eq!(v2.x(), refracted.unwrap().x());
        assert_approx_eq!(v2.y(), refracted.unwrap().y());
        assert_approx_eq!(v2.z(), refracted.unwrap().z());
    }

    #[test]
    fn test_refract() {
        let v1 = dir(0.5, 3f32.sqrt() / 2., 0.);
        let v2 = dir(1., 1., 0.).unit_vector();
        let n = dir(0., -1., 0.);

        let refracted = refract(v1, n, 2f32.sqrt());
        assert!(refracted.is_some());

        assert_approx_eq!(v2.x(), refracted.unwrap().x());
        assert_approx_eq!(v2.y(), refracted.unwrap().y());
        assert_approx_eq!(v2.z(), refracted.unwrap().z());
    }

    #[test]
    fn test_dielectric_refract_entering() {
        let r_in = Ray::new(Pos::zero(), dir(1., 1., 0.).unit_vector(), 0.);
        let mat = Dielectric::new(2f32.sqrt());
        let rec = HitRecord {
            t: 1.,
            p: pos(1., 1., 0.),
            normal: dir(0., -1., 0.),
            mat: &mat,
        };

        let scatter = mat.scatter(&r_in, &rec);
        assert!(scatter.is_some());
        let scattered = scatter.unwrap().scattered.direction();

        let v2 = dir(0.5, 3f32.sqrt() / 2., 0.);
        assert_approx_eq!(v2.x(), scattered.x());
        assert_approx_eq!(v2.y(), scattered.y());
        assert_approx_eq!(v2.z(), scattered.z());
    }

    #[test]
    fn test_dielectric_refract_exiting() {
        let r_in = Ray::new(Pos::zero(), dir(0.5, 3f32.sqrt() / 2., 0.), 0.);
        let mat = Dielectric::new(2f32.sqrt());
        let rec = HitRecord {
            t: 1.,
            p: pos(0.5, 3f32.sqrt() / 2., 0.),
            normal: dir(0., 1., 0.),
            mat: &mat,
        };

        let scatter = mat.scatter(&r_in, &rec);
        assert!(scatter.is_some());
        let scattered = scatter.unwrap().scattered.direction();

        let v2 = dir(1., 1., 0.).unit_vector();
        assert_approx_eq!(v2.x(), scattered.x());
        assert_approx_eq!(v2.y(), scattered.y());
        assert_approx_eq!(v2.z(), scattered.z());
    }
}
