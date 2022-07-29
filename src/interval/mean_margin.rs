use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Sub};

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

impl<T> Add<T> for MeanMarginInterval<T>
where
    T: Copy + Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self {
            mean: self.mean + rhs,
            margin: self.margin,
        }
    }
}

impl<T> Mul<T> for MeanMarginInterval<T>
where
    T: Copy + Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            mean: self.mean * rhs,
            margin: self.margin * rhs,
        }
    }
}

impl<T> PartialEq for MeanMarginInterval<T> {
    #[must_use]
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl<T> PartialOrd for MeanMarginInterval<T>
where
    T: Copy + Sub<Output = T> + Add<Output = T> + PartialOrd,
{
    #[must_use]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.upper() < other.lower() {
            Some(Ordering::Less)
        } else if self.lower() > other.upper() {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}
