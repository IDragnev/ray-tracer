mod ray;
mod interaction;

pub use ray::Ray;
pub use interaction::Interaction;

use crate::math::Interval;
use crate::aabb::AABB;

pub trait Hittable {
    fn hit(&self, ray: &Ray, hit_interval: &Interval<f32>) -> Option<Interaction>;
    fn bounding_box(&self, time_interval: &Interval<f32>) -> Option<AABB>;
}