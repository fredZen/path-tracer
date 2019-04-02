use crate::vec3::{Dir, Float, Pos};

#[derive(Clone, Debug, PartialEq)]
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

    pub fn point_at(&self, t: Float) -> Pos {
        self.a + t * self.b
    }
}
