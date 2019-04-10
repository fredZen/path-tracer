use crate::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Ray {
    a: Pos,
    b: Dir,
    time: Float,
}

impl Ray {
    pub fn new(a: Pos, b: Dir, ti: Float) -> Self {
        Ray { a, b, time: ti }
    }

    pub fn origin(&self) -> Pos {
        self.a
    }

    pub fn direction(&self) -> Dir {
        self.b
    }

    pub fn time(&self) -> Float {
        self.time
    }

    pub fn point_at(&self, t: Float) -> Pos {
        self.a + t * self.b
    }
}
