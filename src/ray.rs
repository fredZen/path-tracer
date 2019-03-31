use crate::vec3::{Dir, Float, Pos};

pub struct Ray {
    a: Pos,
    b: Dir,
}

impl Ray {
    pub fn new(a: Pos, b: Dir) -> Self {
        Ray { a, b }
    }

    pub fn origin(&self) -> Pos {
        self.a
    }

    pub fn direction(&self) -> Dir {
        self.b
    }

    fn point_at(&self, t: Float) -> Pos {
        self.a + t * self.b
    }
}
