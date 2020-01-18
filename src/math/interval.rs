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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interval_models_open_intervals() {
        assert!(Interval::new(0, 0).is_none());
        assert!(Interval::new(0, 1).is_some());
    }

    #[test]
    fn overlap_with_non_overlapping_intervals() {
        let first = Interval::new(0, 1).unwrap();
        let second = Interval::new(1, 5).unwrap();

        assert!(first.overlap_with(&second).is_none());
        assert!(second.overlap_with(&first).is_none());
    }

    #[test]
    fn overlap_with_overlapping_intervals() {
        let first = Interval::new(0, 2).unwrap();
        let second = Interval::new(1, 100).unwrap();

        let third = first.overlap_with(&second);

        assert!(third.is_some());
        assert_eq!(third.unwrap().min(), 1);
        assert_eq!(third.unwrap().max(), 2);
    }
}