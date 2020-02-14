mod dielectric;
mod lambertian;
mod metal;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;

use crate::{
    core,
    math,
};

pub struct ScatterResult {
    pub attenuation: math::Vec3,
    pub scattered_ray: core::Ray,
}

pub trait Material {
    fn scatter(&self, ray: &core::Ray, hit_record: &core::HitRecord) -> Option<ScatterResult>;
}