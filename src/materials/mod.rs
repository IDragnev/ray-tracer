mod material;
mod dielectric;
mod lambertian;
mod metal;
mod diffuse_light;
mod isotropic;

pub use material::{
    Material,
    ScatterResult,
};
pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;
pub use diffuse_light::DiffuseLight;
pub use isotropic::Isotropic;