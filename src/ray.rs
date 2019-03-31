use crate::vec3::{Dir, Float, Pos};

struct Ray {
    a: Pos,
    b: Dir,
}

impl Ray {
    fn new(a: Pos, b: Dir) -> Self {
        Ray { a, b }
    }

    fn origin(&self) -> Pos {
        self.a
    }

    fn direction(&self) -> Dir {
        self.b
    }

    fn point_at(&self, t: Float) -> Pos {
        self.a + t * self.b
    }
}
