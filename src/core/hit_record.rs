use crate::{
    materials::{
        Material,
    },
    textures::{
        TextureCoordinates,
    },
    math,
};

#[derive(Copy, Clone)]
pub struct HitRecord<'a> {
    pub t: f32,
    pub hit_point: math::Point3,
    pub normal: math::Vec3,
    pub material: &'a dyn Material,
    pub uv: TextureCoordinates,
}