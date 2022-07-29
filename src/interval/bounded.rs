use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Sub};

use num_traits::One;

use crate::{CenteredInterval, Interval};

/// Interval represented by its lower and upper bounds.
#[must_use]
#[derive(Clone)]
pub struct BoundedInterval<T> {
    pub(crate) lower: T,
    pub(crate) upper: T,
}

impl<T> Interval<T> for BoundedInterval<T>
where
    T: Copy + One + Add<Output = T> + Sub<Output = T> + Div<Output = T> + PartialOrd,
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

    #[inline]
    fn contains(&self, point: T) -> bool {
        point >= self.lower && point <= self.upper
    }
}

impl<T> From<CenteredInterval<T>> for BoundedInterval<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + PartialOrd,
{
    fn from(interval: CenteredInterval<T>) -> Self {
        Self {
            lower: interval.lower(),
            upper: interval.upper(),
        }
    }
}

impl<T> Add<T> for BoundedInterval<T>
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

impl<T> Mul<T> for BoundedInterval<T>
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

impl<T> PartialEq<Self> for BoundedInterval<T> {
    #[must_use]
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl<T: PartialOrd<T>> PartialOrd<Self> for BoundedInterval<T> {
    #[must_use]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.upper < other.lower {
            Some(Ordering::Less)
        } else if self.lower > other.upper {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

impl<T> PartialEq<T> for BoundedInterval<T> {
    fn eq(&self, _other: &T) -> bool {
        false
    }
}

impl<T: PartialOrd<T>> PartialOrd<T> for BoundedInterval<T> {
    #[must_use]
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        if &self.upper < other {
            Some(Ordering::Less)
        } else if &self.lower > other {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}
