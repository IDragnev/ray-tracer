mod dielectric;
mod lambertian;
mod metal;
mod diffuse_light;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;
pub use diffuse_light::DiffuseLight;

use crate::{
    core,
    math,
    textures::{
        TextureCoordinates,
    },
};

#[derive(Copy, Clone)]
pub struct ScatterResult {
    pub attenuation: math::Vec3,
    pub scattered_ray: core::Ray,
}

pub trait Material {
    fn scatter(&self, ray: &core::Ray, hit_record: &core::HitRecord) -> Option<ScatterResult>;
    
    fn emitted(&self, _: &TextureCoordinates, _p: &math::Point3) -> math::Vec3 {
        math::vec3(0.0, 0.0, 0.0)
    }
}