use std::ops::{Add, Div, Sub};

use num_traits::One;

use crate::{Interval, LowerUpperInterval};

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

impl<T> From<LowerUpperInterval<T>> for MeanMarginInterval<T>
where
    T: Copy + Add<Output = T> + Div<Output = T> + Sub<Output = T> + One,
{
    fn from(interval: LowerUpperInterval<T>) -> Self {
        Self {
            mean: interval.mean(),
            margin: interval.margin(),
        }
    }
}
