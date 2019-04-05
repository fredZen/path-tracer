use crate::vec3::{Col, Float};
use image::{ImageBuffer, Rgb};
use rayon::prelude::*;

pub struct Pixbuf {
    w: usize,
    h: usize,
    pixels: Vec<Col>,
}

fn as_u8(f: Float) -> u8 {
    if f < 1. {
        (256. * f) as u8
    } else {
        255
    }
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

    fn get(&self, x: usize, y: usize) -> Col {
        debug_assert!(x < self.w);
        debug_assert!(y < self.h);
        self.pixels[x + self.w * y]
    }

    pub fn as_image(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut img = ImageBuffer::new(self.w as u32, self.h as u32);
        for (i, j, p) in img.enumerate_pixels_mut() {
            let c = self.get(i as usize, j as usize);
            let col = Col::new(c.r().sqrt(), c.g().sqrt(), c.b().sqrt());

            let ir = as_u8(col.r());
            let ig = as_u8(col.g());
            let ib = as_u8(col.b());
            *p = Rgb([ir, ig, ib]);
        }
        img
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

impl std::ops::DivAssign<usize> for Pixbuf {
    fn div_assign(&mut self, i: usize) {
        self.pixels.par_iter_mut().for_each(|c| *c /= i as Float);
    }
}
