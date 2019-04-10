use crate::prelude::*;
use rand::prelude::*;

pub struct Camera {
    lower_left_corner: Pos,
    horizontal: Dir,
    vertical: Dir,
    origin: Pos,
    u: Dir,
    v: Dir,
    lens_radius: Float,
    time0: Float,
    time1: Float,
}

fn random_in_unit_disk() -> Dir {
    let mut rng = thread_rng();
    loop {
        let p = 2. * dir(rng.gen(), rng.gen(), 0.) - dir(1., 1., 0.);
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
        t0: Float,
        t1: Float,
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
            time0: t0,
            time1: t1,
        }
    }

    #[inline]
    pub fn get_ray(&self, u: Float, v: Float) -> Ray {
        let mut rng = thread_rng();

        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        let time = self.time0 + rng.gen::<Float>() * (self.time1 - self.time0);

        Ray::new(
            self.origin + offset,
            self.lower_left_corner - self.origin + u * self.horizontal + v * self.vertical - offset,
            time,
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
            pos(0., 0., -1.),
            dir(0., 1., 0.),
            90.,
            2.,
            0.,
            1.,
            0.,
            0.,
        );
        assert_eq!(Pos::zero(), cam.origin);
        assert_eq!(pos(-2., -1., -1.), cam.lower_left_corner);
        assert_eq!(dir(4., 0., 0.), cam.horizontal);
        assert_eq!(dir(0., 2., 0.), cam.vertical);
    }
}
