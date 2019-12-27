use crate::materials::Material;
use crate::math::{Point3, Vec3};

pub struct Interaction<'a> {
    pub t: f32,
    pub hit_point: Point3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
}