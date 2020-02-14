mod ray;
mod hit_record;

pub use ray::Ray;
pub use hit_record::HitRecord;

use crate::math::Interval;
use crate::aabb::AABB;

pub trait Hittable {
    fn hit(&self, ray: &Ray, hit_interval: &Interval<f32>) -> Option<HitRecord>;
    fn bounding_box(&self, time_interval: &Interval<f32>) -> Option<AABB>;
}