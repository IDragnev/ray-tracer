use crate::core::{
    Ray,
    Interaction,
    Hittable,
};
use crate::aabb::{
    self,
    AABB,
};
use crate::math::{
    Interval,
};

type BoxedHittable = Box<dyn Hittable>;

pub struct Node {
    left: BoxedHittable,
    right: BoxedHittable,
    pub bounding_box: AABB,
}

impl Node {
    pub fn new(mut hittables: Vec<BoxedHittable>, time_interval: &Interval<f32>) -> Node {
        let axis = crate::random(0, 3);
        hittables.sort_unstable_by(|a, b| box_compare(&**a, &**b, axis));
        
        let (left, right) = match hittables.len() {
            0 | 1 => panic!("incorrect list"),
            2 => (hittables.pop().unwrap(), hittables.pop().unwrap()),
            3 => (hittables.pop().unwrap(), Box::new(Node::new(hittables, time_interval)) as BoxedHittable),
            n => { 
                let left : BoxedHittable = Box::new(Node::new(hittables.drain(..n / 2).collect(), time_interval));
                let right: BoxedHittable = Box::new(Node::new(hittables, time_interval));
                (left, right)
            },
        };
        let box_left = left.bounding_box(time_interval);
        let box_right = right.bounding_box(time_interval);
        let bounding_box = match (box_left, box_right) {
            (Some(x), Some(y)) => aabb::surrounding_box(&x, &y),
            _ => panic!("Hittable with no bounding box given to BVH Node"),
        };
        
        Node {
            left,
            right,
            bounding_box,
        }
    }
}

fn box_compare(left: &dyn Hittable, right: &dyn Hittable, dim: usize) -> std::cmp::Ordering {
    let interval = Interval::new(0.0, 0.3).unwrap();
    match (left.bounding_box(&interval), right.bounding_box(&interval)) {
        (Some(left_box), Some(right_box)) => {
            left_box.min[dim].partial_cmp(&right_box.min[dim])
                             .expect("NaN value in AABB")
        },
        _ => panic!("Hittable with no bounding box given to BVH Node"),
    } 
}

impl Hittable for Node {
    fn hit(&self, ray: &Ray, hit_interval: &Interval<f32>) -> Option<Interaction> {
        if !self.bounding_box.hit(ray, hit_interval) {
            return None;
        }
        
        let left_interaction = self.left.hit(ray, hit_interval);
        let right_interaction = self.right.hit(ray, hit_interval);
        match (left_interaction, right_interaction) {
            (Some(left), Some(right)) => {
                Some(if left.t < right.t { left } else { right })
            },
            (Some(x), None) => Some(x),
            (None, Some(x)) => Some(x),
            (None, None) => None,
        }
    }

    fn bounding_box(&self, _: &Interval<f32>) -> Option<AABB> {
        Some(self.bounding_box)
    }
}