#[derive(Copy, Clone)]
pub struct Interval<T: Copy + Clone + PartialOrd> {
    min: T,
    max: T,
}

impl<T: Copy + Clone + PartialOrd> Interval<T> {
    pub fn new(min: T, max: T) -> Option<Self> {
        if min < max {
            Some(Interval {
                min, 
                max,
            }) 
        }
        else {
            None
        }
    }
    
    pub fn overlap_with(&self, other: &Interval<T>) -> Option<Self> {
        let min = if self.min > other.min { self.min } else { other.min };
        let max = if self.max < other.max { self.max } else { other.max };
        Interval::new(min, max)
    }

    pub fn min(&self) -> T {
        self.min
    }

    pub fn max(&self) -> T {
        self.max
    }
}