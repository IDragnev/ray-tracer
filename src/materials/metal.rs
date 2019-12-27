use crate::math;
use crate::core::{
    Ray,
    Interaction,
};
use crate::materials::{
    self,
    Material,
    Result,
};

pub struct Metal {
    albedo: math::Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: math::Vec3, fuzz: f32) -> Metal {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Metal {
            albedo,
            fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, interaction: &Interaction) -> Option<Result> {
        use math::{InnerSpace, EuclideanSpace};
        use materials::random_point_from_unit_sphere;
        let reflected_dir = math::reflected(&ray.direction.normalize(), &interaction.normal);
        let is_angle_acute = math::dot(reflected_dir, interaction.normal) > 0.0;
        if is_angle_acute {
            let direction = reflected_dir + self.fuzz * random_point_from_unit_sphere().to_vec();
            Some(Result{
                scattered_ray: Ray::new(interaction.hit_point, direction),
                attenuation: self.albedo,
            })
        }
        else {
            None
        }
    }
}