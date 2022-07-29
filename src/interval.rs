//! Interval trait and structures.

pub mod lower_upper;
pub mod mean_margin;

pub use self::lower_upper::*;
pub use self::mean_margin::*;

pub trait Interval<F> {
    /// Returns the interval's mean.
    #[must_use]
    fn mean(&self) -> F;

    /// Returns the interval's margin.
    #[must_use]
    fn margin(&self) -> F;

    /// Returns the interval's lower bound.
    #[must_use]
    fn lower(&self) -> F;

    /// Returns the interval's upper bound.
    #[must_use]
    fn upper(&self) -> F;

    /// Returns the interval's bounds as a 2-tuple.
    #[must_use]
    fn bounds(&self) -> (F, F);
}
