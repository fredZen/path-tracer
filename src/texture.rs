use crate::prelude::*;
use std::fmt::Debug;

pub trait Texture: Debug {
    fn value(&self, u: Float, v: Float, p: &Pos) -> Col;
}

pub type TextureBox = Box<dyn Texture + Send + Sync>;

#[derive(Debug)]
struct ConstantTexture {
    colour: Col,
}

impl Texture for ConstantTexture {
    #[inline]
    fn value(&self, _u: Float, _v: Float, _p: &Pos) -> Col {
        self.colour
    }
}

pub fn constant_texture(colour: Col) -> TextureBox {
    Box::new(ConstantTexture { colour })
}
