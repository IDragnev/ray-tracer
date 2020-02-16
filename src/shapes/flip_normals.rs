use crate::{
    math::{
        Interval,
    },
    core::{
        Hittable, 
        HitRecord,
        Ray,
    },
    aabb::{
        AABB,
    },
};

pub struct FlipNormals<T: Hittable> {
    hittable: T,
}

impl<T: Hittable> FlipNormals<T> {
    pub fn new(hittable: T) -> Self {
        Self {
            hittable,
        }
    }
}

impl<T: Hittable> Hittable for FlipNormals<T> {
    fn hit(&self, ray: &Ray, hit_interval: &Interval<f32>) -> Option<HitRecord> {
        self.hittable.hit(ray, hit_interval)
        .map(|hit_rec| {
            HitRecord {
                t: hit_rec.t,
                hit_point: hit_rec.hit_point,
                normal: -hit_rec.normal,
                material: hit_rec.material,
                uv: hit_rec.uv,
            }
        })
    }
    fn bounding_box(&self, time_interval: &Interval<f32>) -> Option<AABB> {
        self.hittable.bounding_box(time_interval)
    }
}