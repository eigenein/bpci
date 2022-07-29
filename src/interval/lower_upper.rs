use std::ops::{Add, Div, Mul, Sub};

use num_traits::One;

use crate::{Interval, MeanMarginInterval};

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

impl<T> From<MeanMarginInterval<T>> for LowerUpperInterval<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T>,
{
    fn from(interval: MeanMarginInterval<T>) -> Self {
        Self {
            lower: interval.lower(),
            upper: interval.upper(),
        }
    }
}

impl<T> Add<T> for LowerUpperInterval<T>
where
    T: Copy + Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self {
            lower: self.lower + rhs,
            upper: self.upper + rhs,
        }
    }
}

impl<T> Mul<T> for LowerUpperInterval<T>
where
    T: Copy + Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            lower: self.lower * rhs,
            upper: self.upper * rhs,
        }
    }
}
