use std::ops::{Add, Sub};

use crate::Interval;

/// Represents an interval with its mean and margin.
#[must_use]
#[derive(Clone)]
pub struct MeanMarginInterval<T> {
    pub mean: T,
    pub margin: T,
}

impl<T: Add<Output = T> + Sub<Output = T> + Copy> Interval<T> for MeanMarginInterval<T> {
    #[inline]
    fn mean(&self) -> T {
        self.mean
    }

    #[inline]
    fn margin(&self) -> T {
        self.margin
    }

    #[inline]
    fn lower(&self) -> T {
        self.mean - self.margin
    }

    #[inline]
    fn upper(&self) -> T {
        self.mean + self.margin
    }

    #[inline]
    fn bounds(&self) -> (T, T) {
        (self.lower(), self.upper())
    }
}
