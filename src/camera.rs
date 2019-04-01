use crate::ray::Ray;
use crate::vec3::{Dir, Float, Pos};

pub struct Camera {
    lower_left_corner: Dir,
    horizontal: Dir,
    vertical: Dir,
    origin: Pos,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            lower_left_corner: Dir::new(-2., -1., -1.),
            horizontal: Dir::new(4., 0., 0.),
            vertical: Dir::new(0., 2., 0.),
            origin: Pos::new(0., 0., 0.),
        }
    }

    pub fn get_ray(&self, u: Float, v: Float) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical,
        )
    }
}
