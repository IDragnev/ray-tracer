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

pub struct DiffuseLight {
    emit: Box<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(emit: Box<dyn Texture>) -> Self {
        DiffuseLight{
            emit,
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _: &Ray, _: &HitRecord) -> Option<ScatterResult> {
        None
    }

    fn emitted(&self, uv: &TextureCoordinates, p: &math::Point3) -> math::Vec3 {
        self.emit.value(uv, p)
    }
}