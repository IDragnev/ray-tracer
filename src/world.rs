use crate::math::Interval;
use crate::core::{
    Ray,
    Interaction,
    Hittable,
};
use crate::aabb::AABB;
use crate::bvh;

pub struct World {
    root: bvh::Node,
}

impl World {
    pub fn new(hittables: Vec<Box<dyn Hittable>>, time_interval: &Interval<f32>) -> Self {
        World {
            root: bvh::Node::new(hittables, time_interval),
        }
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, hit_interval: &Interval<f32>) -> Option<Interaction> {
        self.root.hit(ray, hit_interval)
    }

    fn bounding_box(&self, time_interval: &Interval<f32>) -> Option<AABB> {
        self.root.bounding_box(time_interval)
    }
}