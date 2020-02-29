use crate::{
    math::{
        Interval,
    },
    aabb::{
        AABB,
    },
    core::{
        Ray, 
        HitRecord,
    },
};

pub trait Hittable {
    fn hit(&self, ray: &Ray, hit_interval: &Interval<f32>) -> Option<HitRecord>;
    fn bounding_box(&self, time_interval: &Interval<f32>) -> Option<AABB>;
}