use crate::math;
use crate::materials::{
    Material,
    Result,
};
use crate::core::{
    Interaction,
    Ray,
};

pub struct Dielectric {
    pub refractive_index: f32,
}

impl Dielectric {
    pub fn new(refractive_index: f32) -> Dielectric {
        Dielectric {
            refractive_index,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, interaction: &Interaction) -> Option<Result> {
        use math::{InnerSpace, vec3};
        use crate::random_float_from_0_to_1;
           
        let direction_normal_dot = math::dot(ray.direction, interaction.normal);
        let (outward_normal, ni_over_nt, cosine) = 
            if direction_normal_dot > 0.0 {
                let cosine = self.refractive_index * direction_normal_dot / ray.direction.magnitude();
                (-interaction.normal, self.refractive_index, cosine)
            }
            else {
                let cosine = -direction_normal_dot / ray.direction.magnitude();
                (interaction.normal, 1.0 / self.refractive_index, cosine)
            };                
        let (refracted_direction, reflection_coefficient) = 
            if let Some(refracted_direction) = math::refracted(&ray.direction, &outward_normal, ni_over_nt) {
                (refracted_direction, math::schlick(cosine, self.refractive_index))
            }
            else {
                (vec3(0.0, 0.0, 0.0), 1.0)
            };
        let direction = if random_float_from_0_to_1() < reflection_coefficient { 
            math::reflected(&ray.direction, &interaction.normal) 
        } 
        else { 
            refracted_direction
        };

        Some(Result{
            scattered_ray: Ray::new(interaction.hit_point, direction, ray.time),
            attenuation: vec3(1.0, 1.0, 1.0),
        })
    }
}