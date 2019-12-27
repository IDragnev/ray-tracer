use crate::math;
use crate::materials::{
    self,
    Result,
    Material,
};
use crate::core::{
    Ray,
    Interaction,
};

pub struct Lambertian {
    albedo: math::Vec3,
}

impl Lambertian {
    pub fn new(albedo: math::Vec3) -> Lambertian {
        Lambertian {
            albedo,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, interaction: &Interaction) -> Option<Result> {
        use materials::random_point_from_unit_sphere;
        use math::EuclideanSpace;
        let tangent_unit_sphere_center = interaction.hit_point + interaction.normal;
        let target_point = tangent_unit_sphere_center + random_point_from_unit_sphere().to_vec();
        let direction = target_point - interaction.hit_point;
        Some(Result{
            attenuation: self.albedo,
            scattered_ray: Ray::new(interaction.hit_point, direction),
        })
    }
}
