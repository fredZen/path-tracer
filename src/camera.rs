use crate::ray::Ray;
use crate::vec3::{Dir, Float, Pos, Vector};
use rand::prelude::*;
use std::f32::consts::*;

pub struct Camera {
    lower_left_corner: Pos,
    horizontal: Dir,
    vertical: Dir,
    origin: Pos,
    u: Dir,
    v: Dir,
    lens_radius: Float,
}

fn random_in_unit_disk() -> Dir {
    let mut rng = thread_rng();
    loop {
        let p = 2. * Dir::new(rng.gen(), rng.gen(), 0.) - Dir::new(1., 1., 0.);
        if p.squared_length() < 1. {
            return p;
        }
    }
}

impl Camera {
    /// vfov is top to bottom in degrees
    pub fn new(
        look_from: Pos,
        look_at: Pos,
        up: Dir,
        vfov: Float,
        aspect: Float,
        aperture: Float,
        focus_dist: Float,
    ) -> Camera {
        let theta = vfov * PI / 180. / 2.;
        let half_height = theta.tan();
        let half_width = aspect * half_height;
        let w = (look_from - look_at).unit_vector();
        let u = up.cross(w).unit_vector();
        let v = w.cross(u);

        Camera {
            lower_left_corner: look_from - focus_dist * (half_width * u + half_height * v + w),
            horizontal: 2. * half_width * focus_dist * u,
            vertical: 2. * half_height * focus_dist * v,
            origin: look_from,
            u,
            v,
            lens_radius: aperture / 2.,
        }
    }

    #[inline]
    pub fn get_ray(&self, u: Float, v: Float) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner - self.origin + u * self.horizontal + v * self.vertical - offset,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_creation() {
        let cam = Camera::new(
            Pos::zero(),
            Pos::new(0., 0., -1.),
            Dir::new(0., 1., 0.),
            90.,
            2.,
            0.,
            1.,
        );
        assert_eq!(Pos::zero(), cam.origin);
        assert_eq!(Pos::new(-2., -1., -1.), cam.lower_left_corner);
        assert_eq!(Dir::new(4., 0., 0.), cam.horizontal);
        assert_eq!(Dir::new(0., 2., 0.), cam.vertical);
    }
}
