pub type Vec3 = cgmath::Vector3<f32>;
pub type Point3 = cgmath::Point3<f32>;

pub use cgmath::vec3;
pub use cgmath::dot;
pub use cgmath::EuclideanSpace;
pub use cgmath::prelude::InnerSpace;
pub use cgmath::prelude::VectorSpace;

#[derive(Copy, Clone)]
pub struct Interval<T: Copy + Clone> {
    pub min: T,
    pub max: T,
}

impl<T: Copy + Clone + PartialOrd> Interval<T> {
    pub fn new(min: T, max: T) -> Option<Self> {
        if min <= max {
            Some(Interval {
                min, 
                max,
            }) 
        }
        else {
            None
        }
    }

    pub fn with_max(&self, max: T) -> Option<Self> {
        Interval::new(self.min, max)
    }

    pub fn with_min(&self, min: T) -> Option<Self> {
        Interval::new(min, self.max)
    }
}