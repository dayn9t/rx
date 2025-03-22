use serde::{Deserialize, Serialize};
use std::ops::Sub;

#[derive(Default, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Range<T> {
    pub min: T,
    pub max: T,
}

impl<T: PartialOrd> Range<T> {
    // Creates a new Range
    pub fn new(min: T, max: T) -> Self {
        assert!(min <= max, "min should be less than or equal to max");
        Range { min, max }
    }

    // Checks if a value is within the range
    pub fn contains(&self, value: &T) -> bool {
        *value >= self.min && *value <= self.max
    }

    // Returns the length of the range
    pub fn len(&self) -> T
    where
        T: Sub<Output = T> + Copy,
    {
        self.max - self.min
    }
}

pub type RangeI = Range<i32>;
pub type RangeF = Range<f32>;
pub type RangeI64 = Range<i64>;
pub type RangeF64 = Range<f64>;

#[cfg(test)]
mod tests {
    use super::Range;

    #[test]
    fn test_new() {
        let range = Range::new(1, 10);
        assert_eq!(range.min, 1);
        assert_eq!(range.max, 10);
    }

    #[test]
    fn test_contains() {
        let range = Range::new(1, 10);
        assert!(range.contains(&5));
        assert!(!range.contains(&0));
        assert!(!range.contains(&11));
    }

    #[test]
    fn test_length() {
        let range = Range::new(1, 10);
        assert_eq!(range.len(), 9);
    }
}
