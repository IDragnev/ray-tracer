#[derive(Copy, Clone)]
pub struct Interval<T: Copy + Clone + PartialOrd> {
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