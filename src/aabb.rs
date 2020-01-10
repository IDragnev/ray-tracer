use crate::math::{
    Point3,
    Interval,
};
use crate::core::{
    Ray,
};

#[derive(Copy, Clone)]
pub struct AABB {
    pub min: Point3,
    pub max: Point3,
}

impl AABB {
    pub fn hit(&self, ray: &Ray, hit_interval: &Interval<f32>) -> bool {
        let mut common_interval = *hit_interval;
        for d in 0..3 {
            let inv_dir = 1.0 / ray.direction[d];
            let slab = {
                let t0 = (self.min[d] - ray.origin[d]) * inv_dir;
                let t1 = (self.max[d] - ray.origin[d]) * inv_dir;
                if inv_dir < 0.0 { Interval::new(t1, t0) } else { Interval::new(t0, t1) }
            };
            if slab.is_none() { return false; } //in case (t0, t1) is (-inf, -inf) or (inf, inf)
            if let Some(interval) = common_interval.overlap_with(&slab.unwrap()) {
                common_interval = interval;
            } 
            else {
                return false;
            }
        }
        true
    }
}

pub fn surrounding_box(a: &AABB, b: &AABB) -> AABB {
    let ffmin = |u, v| if u < v { u } else { v };
    let ffmax = |u, v| if u > v { u } else { v };
    let min = Point3::new( 
        ffmin(a.min.x, b.min.x),
        ffmin(a.min.y, b.min.y),
        ffmin(a.min.z, b.min.z));
    let max = Point3::new( 
        ffmax(a.max.x, b.max.x),
        ffmax(a.max.y, b.max.y),
        ffmax(a.max.z, b.max.z));
    AABB {
        min,
        max,
    }
}