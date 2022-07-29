use std::ops::{Add, Div, Sub};

use num_traits::One;

use crate::Interval;

/// Interval represented by its lower and upper bounds.
#[must_use]
#[derive(Clone)]
pub struct LowerUpperInterval<T> {
    pub(crate) lower: T,
    pub(crate) upper: T,
}

impl<T: Copy + One + Add<Output = T> + Sub<Output = T> + Div<Output = T>> Interval<T>
    for LowerUpperInterval<T>
{
    #[inline]
    fn mean(&self) -> T {
        (self.lower + self.upper) / (T::one() + T::one())
    }

    #[inline]
    fn margin(&self) -> T {
        (self.upper - self.lower) / (T::one() + T::one())
    }

    #[inline]
    fn lower(&self) -> T {
        self.lower
    }

    #[inline]
    fn upper(&self) -> T {
        self.upper
    }

    #[inline]
    fn bounds(&self) -> (T, T) {
        (self.lower, self.upper)
    }
}
