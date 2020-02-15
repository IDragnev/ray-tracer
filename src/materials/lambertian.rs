use crate::{
    math,
    materials::{
        ScatterResult,
        Material,
    },
    core::{
        Ray,
        HitRecord,
    },
    textures::{
        Texture,
        TextureCoordinates,
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
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
        use math::{random_point_from_unit_sphere, EuclideanSpace};
        
        let tangent_unit_sphere_center = hit_record.hit_point + hit_record.normal;
        let target_point = tangent_unit_sphere_center + random_point_from_unit_sphere().to_vec();
        let direction = target_point - hit_record.hit_point;
        let uv = TextureCoordinates::zero();
        Some(ScatterResult{
            attenuation: self.albedo.value(&uv, &hit_record.hit_point),
            scattered_ray: Ray::new(hit_record.hit_point, direction, ray.time),
        })
    }
}
