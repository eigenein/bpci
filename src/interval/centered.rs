use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Sub};

use num_traits::One;

use crate::{BoundedInterval, Interval};

/// Represents an interval with its mean and margin.
#[must_use]
#[derive(Clone)]
pub struct CenteredInterval<T> {
    pub mean: T,
    pub margin: T,
}

impl<T> Interval<T> for CenteredInterval<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy + PartialOrd,
{
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

    #[inline]
    fn contains(&self, point: T) -> bool {
        point >= self.lower() && point <= self.upper()
    }
}

impl<T> From<BoundedInterval<T>> for CenteredInterval<T>
where
    T: Copy + Add<Output = T> + Div<Output = T> + Sub<Output = T> + One + PartialOrd,
{
    fn from(interval: BoundedInterval<T>) -> Self {
        Self {
            mean: interval.mean(),
            margin: interval.margin(),
        }
    }
}

impl<T> Add<T> for CenteredInterval<T>
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

impl<T> Mul<T> for CenteredInterval<T>
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

impl<T> PartialEq for CenteredInterval<T> {
    #[must_use]
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl<T> PartialOrd for CenteredInterval<T>
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
