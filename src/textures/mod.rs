mod constant_texture;
mod checker_texture;
mod noise_texture;

pub use constant_texture::ConstantTexture;
pub use checker_texture::CheckerTexture;
pub use noise_texture::NoiseTexture;
 
use crate::{
    math::{
        Point3,
        Vec3,
    },
};

pub trait Texture {
    fn value(&self, uv: (f32, f32), p: &Point3) -> Vec3;
}
