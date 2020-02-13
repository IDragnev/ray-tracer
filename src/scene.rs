use crate::{
    math::{
        Interval,
    },
    core::{
        Ray,
        Interaction,
        Hittable,
    },
    aabb::{
        AABB,
    },
    bvh,
};

pub struct Scene {
    root: bvh::Node,
}

impl Scene {
    pub fn new(hittables: Vec<Box<dyn Hittable>>, time_interval: &Interval<f32>) -> Self {
        Scene {
            root: bvh::Node::new(hittables, time_interval),
        }
    }
}

impl Hittable for Scene {
    fn hit(&self, ray: &Ray, hit_interval: &Interval<f32>) -> Option<Interaction> {
        self.root.hit(ray, hit_interval)
    }

    fn bounding_box(&self, time_interval: &Interval<f32>) -> Option<AABB> {
        self.root.bounding_box(time_interval)
    }
}