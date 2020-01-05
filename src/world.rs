use crate::math::Interval;
use crate::core::{
    Ray,
    Interaction,
    Hittable,
};
use crate::aabb::{
    self,
    AABB,
};

pub struct World {
    hittables: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn new(hittables: Vec<Box<dyn Hittable>>) -> Self {
        World {
            hittables,
        }
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, hit_interval: &Interval<f32>) -> Option<Interaction> {
        let mut interval = *hit_interval;
        let mut result = None;
        for hittable in &self.hittables {
            if let Some(interaction) = hittable.hit(ray, &interval) {
                interval = interval.with_max(interaction.t).expect("invalid interval");
                result = Some(interaction);
            }
        }       
        result
    }

    fn bounding_box(&self, time_interval: &Interval<f32>) -> Option<AABB> {
        let mut result = self.hittables.get(0)?
                         .bounding_box(time_interval)?;
        for hittable in &self.hittables[1..] {
            let b = hittable.bounding_box(time_interval)?;
            result = aabb::surrounding_box(&result, &b);
        }
        Some(result)
    }
}