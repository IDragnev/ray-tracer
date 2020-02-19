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

#[derive(Copy, Clone, Debug)]
pub struct TextureCoordinates {
    pub u: f32,
    pub v: f32,
}

impl TextureCoordinates {
    pub fn zero() -> Self {
        Self {
            u: 0.0, 
            v: 0.0,
        }
    }

    pub fn from_unit_sphere_coordinates(p: &Point3) -> Self {
        const PI: f32 = std::f32::consts::PI;
        let phi = p.z.atan2(p.x);
        let theta = p.y.asin();
        
        Self {
            u: 1.0 - (phi + PI) / (2.0 * PI),
            v: (theta + PI / 2.0) / PI,
        }
    }
}

pub trait Texture {
    fn value(&self, uv: &TextureCoordinates, p: &Point3) -> Vec3;
}
