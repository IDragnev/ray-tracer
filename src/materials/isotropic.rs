use crate::{
    math,
    core::{
        Ray,
        HitRecord,
    },
    materials::{
        Material,
        ScatterResult,
    },
    textures::{
        Texture,
    },
};

pub struct Isotropic {
    albedo: Box<dyn Texture>,
}

impl Isotropic {
    pub fn new(albedo: Box<dyn Texture>) -> Self {
        Self {
            albedo,
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
        use math::EuclideanSpace;

        Some(ScatterResult {
            scattered_ray: Ray::new(hit_record.hit_point, math::random_point_from_unit_sphere().to_vec(), ray.time),
            attenuation: self.albedo.value(&hit_record.uv, &hit_record.hit_point),
        })
    }
}