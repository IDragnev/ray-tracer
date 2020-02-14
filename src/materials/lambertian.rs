use crate::{
    math,
    materials::{
        self,
        Result,
        Material,
    },
    core::{
        Ray,
        HitRecord,
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
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Result> {
        use materials::random_point_from_unit_sphere;
        use math::EuclideanSpace;
        
        let tangent_unit_sphere_center = hit_record.hit_point + hit_record.normal;
        let target_point = tangent_unit_sphere_center + random_point_from_unit_sphere().to_vec();
        let direction = target_point - hit_record.hit_point;
        Some(Result{
            attenuation: self.albedo.value((0.0, 0.0), &hit_record.hit_point),
            scattered_ray: Ray::new(hit_record.hit_point, direction, ray.time),
        })
    }
}
