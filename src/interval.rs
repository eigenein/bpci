use std::ops::{Add, Sub};

/// Represents a resulting interval.
pub struct Interval<T = f64> {
    pub mean: T,
    pub margin: T,
}

impl<T: Sub<Output = T> + Copy> Interval<T> {
    /// Get the lower bound.
    #[must_use]
    pub fn lower(&self) -> T {
        self.mean - self.margin
    }
}

impl<T: Add<Output = T> + Copy> Interval<T> {
    /// Get the upper bound.
    #[must_use]
    pub fn upper(&self) -> T {
        self.mean + self.margin
    }
}

impl<T: Add<Output = T> + Sub<Output = T> + Copy> Interval<T> {
    /// Get the lower and upper bound as a 2-tuple.
    #[must_use]
    pub fn bounds(&self) -> (T, T) {
        (self.lower(), self.upper())
    }
}
