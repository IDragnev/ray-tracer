use crate::{
    math::{
        Point3,
        Vec3,
    },
    textures::{
        Texture,
    },
};

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
    fn value(&self, _: (f32, f32), _: &Point3) -> Vec3 {
        self.colour
    }
}
