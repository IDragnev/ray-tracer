use crate::{
    math,
    materials::{
        self,
        Result,
        Material,
    },
    core::{
        Ray,
        Interaction,
    },
    textures::{
        Texture,
    },
};

pub struct Lambertian {
    albedo: Box<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Box<dyn Texture>) -> Lambertian {
        Lambertian {
            albedo,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, interaction: &Interaction) -> Option<Result> {
        use materials::random_point_from_unit_sphere;
        use math::EuclideanSpace;
        
        let tangent_unit_sphere_center = interaction.hit_point + interaction.normal;
        let target_point = tangent_unit_sphere_center + random_point_from_unit_sphere().to_vec();
        let direction = target_point - interaction.hit_point;
        Some(Result{
            attenuation: self.albedo.value((0.0, 0.0), &interaction.hit_point),
            scattered_ray: Ray::new(interaction.hit_point, direction, ray.time),
        })
    }
}
