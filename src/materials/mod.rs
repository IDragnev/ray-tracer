mod dielectric;
mod lambertian;
mod metal;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;

use crate::core;
use crate::math;

pub struct Result {
    pub attenuation: math::Vec3,
    pub scattered_ray: core::Ray,
}

pub trait Material {
    fn scatter(&self, ray: &core::Ray, interaction: &core::Interaction) -> Option<Result>;
}

fn random_point_from_unit_sphere() -> math::Point3 {
    use crate::math::{vec3, EuclideanSpace, InnerSpace};
    use crate::random_float_from_0_to_1;
    loop {
        let vec = 
            vec3(random_float_from_0_to_1(), random_float_from_0_to_1(), random_float_from_0_to_1())
            .map(|c| 2.0 * c)
            .map(|c| c - 1.0);
        if vec.magnitude() < 1.0 {
            return EuclideanSpace::from_vec(vec);
        }
    }
}