use crate::materials::Material;
use crate::math;

#[derive(Copy, Clone)]
pub struct HitRecord<'a> {
    pub t: f32,
    pub hit_point: math::Point3,
    pub normal: math::Vec3,
    pub material: &'a dyn Material,
}