use crate::{
    math::{
        self,
        Point3, 
        Interval,
    },
    core::{
        Hittable, 
        HitRecord,
        Ray,
    },
    materials::{
        Material,
    },
    aabb::{
        AABB,
    },
    textures::{
        TextureCoordinates,
    }
};

pub struct YZRectangle {
    y0: f32,
    y1: f32,
    z0: f32,
    z1: f32,
    x: f32,
    material: Box<dyn Material>,
}

impl YZRectangle {
    pub fn new(y0: f32, y1: f32, z0: f32, z1: f32, x: f32, material: Box<dyn Material>) -> Self {
        Self {
            y0,
            y1,
            z0,
            z1,
            x,
            material,
        }
    }
}

impl Hittable for YZRectangle {
    fn hit(&self, ray: &Ray, hit_interval: &Interval<f32>) -> Option<HitRecord> {
        let t = (self.x - ray.origin.x) / ray.direction.x;
        if t < hit_interval.min() || t > hit_interval.max() { 
            return None;
        }

        let hit_point = ray.at(t);
        if hit_point.y < self.y0 || hit_point.y > self.y1 || 
           hit_point.z < self.z0 || hit_point.z > self.z1 {
            return None;
        }

        let uv = TextureCoordinates {
            u: (hit_point.y - self.y0) / (self.y1 - self.y0),
            v: (hit_point.z - self.z0) / (self.z1 - self.z0),
        };
        Some(HitRecord{
            t,
            uv,
            hit_point,
            material: self.material.as_ref(),
            normal: math::vec3(1.0, 0.0, 0.0),
        })
    }

    fn bounding_box(&self, _: &Interval<f32>) -> Option<AABB> {
        Some(AABB {
            min: Point3::new(self.x - 0.0001, self.y0, self.z0), 
            max: Point3::new(self.x + 0.0001, self.y1, self.z1),
        })
    }
}