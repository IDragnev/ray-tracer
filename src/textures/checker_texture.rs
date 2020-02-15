use crate::{
    textures::{
        Texture,
        TextureCoordinates,
    },
    math::{
        Point3,
        Vec3,
    }
};

pub struct CheckerTexture {
    pub even: Box<dyn Texture>,
    pub odd:  Box<dyn Texture>,
}

impl Texture for CheckerTexture {
    fn value(&self, uv: &TextureCoordinates, p: &Point3) -> Vec3 {
        let p = p.map(|c| (10.0 * c).sin());
        let sines = p.x * p.y * p.z;
        if sines < 0.0 { 
            self.odd.value(uv, &p) 
        } 
        else { 
            self.even.value(uv, &p) 
        }
    }
}