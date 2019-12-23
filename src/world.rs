use crate::Hittable;
use crate::Ray;
use crate::math::Interval;
use crate::Interaction;

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
}