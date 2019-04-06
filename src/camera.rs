use crate::ray::Ray;
use crate::vec3::{Dir, Float, Pos};
use std::f32::consts::*;

pub struct Camera {
    lower_left_corner: Pos,
    horizontal: Dir,
    vertical: Dir,
    origin: Pos,
}

impl Camera {
    /// vfov is top to bottom in degrees
    pub fn new(vfov: Float, aspect: Float) -> Camera {
        let theta = vfov * PI / 180. / 2.;
        let half_height = theta.tan();
        let half_width = aspect * half_height;
        Camera {
            lower_left_corner: Pos::new(-half_width, -half_height, -1.),
            horizontal: Dir::new(2. * half_width, 0., 0.),
            vertical: Dir::new(0., 2. * half_height, 0.),
            origin: Pos::new(0., 0., 0.),
        }
    }

    #[inline]
    pub fn get_ray(&self, u: Float, v: Float) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner - self.origin + u * self.horizontal + v * self.vertical,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_creation() {
        let cam = Camera::new(90., 2.);
        assert_eq!(Pos::zero(), cam.origin);
        assert_eq!(Pos::new(-2., -1., -1.), cam.lower_left_corner);
        assert_eq!(Dir::new(4., 0., 0.), cam.horizontal);
        assert_eq!(Dir::new(0., 2., 0.), cam.vertical);
    }
}
