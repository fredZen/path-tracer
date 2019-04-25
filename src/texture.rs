mod perlin;

use crate::prelude::*;
use std::fmt::Debug;

pub trait Texture: Debug + TextureClone {
    fn value(&self, u: Float, v: Float, p: Pos) -> Col;
}

pub type TextureBox = Box<dyn Texture + Send + Sync>;

pub trait TextureClone {
    fn box_clone(&self) -> TextureBox;
}

impl<T> TextureClone for T
where
    T: 'static + Texture + Clone + Send + Sync,
{
    fn box_clone(&self) -> TextureBox {
        Box::new(self.clone())
    }
}

impl Clone for TextureBox {
    fn clone(&self) -> TextureBox {
        self.box_clone()
    }
}

#[derive(Clone, Debug)]
struct ConstantTexture {
    colour: Col,
}

impl Texture for ConstantTexture {
    #[inline]
    fn value(&self, _u: Float, _v: Float, _p: Pos) -> Col {
        self.colour
    }
}

pub fn constant_texture(colour: Col) -> TextureBox {
    Box::new(ConstantTexture { colour })
}

#[derive(Clone, Debug)]
struct Checker {
    odd: TextureBox,
    even: TextureBox,
}

impl Texture for Checker {
    #[inline]
    fn value(&self, u: Float, v: Float, p: Pos) -> Col {
        let sines = p.iter().map(|p| (10. * p).sin()).product::<Float>();
        let texture = if sines < 0. { &self.odd } else { &self.even };
        texture.value(u, v, p)
    }
}

pub fn checker(odd: TextureBox, even: TextureBox) -> TextureBox {
    Box::new(Checker { odd, even })
}

pub fn noise_texture(scale: Float) -> TextureBox {
    use perlin::NoiseTexture;
    Box::new(NoiseTexture::new(scale))
}
