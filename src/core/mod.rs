mod ray;
mod interaction;

pub use ray::Ray;
pub use interaction::Interaction;

use crate::math::Interval;

pub trait Hittable {
    fn hit(&self, ray: &Ray, hit_interval: &Interval<f32>) -> Option<Interaction>;
}