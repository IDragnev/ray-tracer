use crate::{
    math::{
        Point3,
        Vec3,
    },
    textures::{
        Texture,
        TextureCoordinates,
    },
};

#[derive(Copy, Clone)]
pub struct ConstantTexture {
    colour: Vec3,
}

impl ConstantTexture {
    pub fn from_rgb(colour: Vec3) -> Self {
        Self {
            colour,    
        }
    }
}

impl Texture for ConstantTexture {
    fn value(&self, _: &TextureCoordinates, _: &Point3) -> Vec3 {
        self.colour
    }
}
