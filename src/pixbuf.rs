use crate::vec3::Col;

pub struct Pixbuf {
    pub w: usize,
    pub h: usize,
    pub pixels: Vec<Col>,
}

impl Pixbuf {
    pub fn new(w: usize, h: usize) -> Pixbuf {
        Pixbuf{
            w,
            h,
            pixels: vec![Col::zero(); w*h],
        }
    }

    pub fn put(&mut self, x: usize, y: usize, c: Col) {
        debug_assert!(x < self.w);
        debug_assert!(y < self.h);
        self.pixels[x + self.w * y] = c;
    }

    pub fn get(&self, x: usize, y: usize) -> Col {
        debug_assert!(x < self.w);
        debug_assert!(y < self.h);
        self.pixels[x + self.w * y]
    }
}

impl std::ops::AddAssign for Pixbuf {
    /// A bit of a wonky add_assign, since it only works for pixbufs with the same dimension,
    /// which constitutes a strengthening of the preconditions. Out of the window goes Barbara
    /// Liskov. Sorry Barb.
    fn add_assign(&mut self, i: Self) {
        assert_eq!(self.w, i.w);
        assert_eq!(self.h, i.h);
        for (d, s) in self.pixels.iter_mut().zip(i.pixels.iter()) {
            *d += *s;
        }
    }
}
