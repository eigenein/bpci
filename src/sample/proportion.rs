use std::fmt::Debug;
use std::ops::Div;

use num_traits::{One, Zero};

use crate::{Error, NSuccessesSample, Result, Sample};

/// Represents a sample with its size and proportion.
pub struct ProportionSample<N, P> {
    pub(crate) size: N,
    pub(crate) proportion: P,
}

impl<N, P> ProportionSample<N, P>
where
    N: Debug + Zero + PartialOrd,
    P: Zero + One + Debug + PartialOrd,
{
    pub fn new(size: N, proportion: P) -> Result<Self> {
        if size < N::zero() {
            return Err(Error::OutOfRange(format!("size `{:?}` must be non-negative", size)));
        }
        if proportion < P::zero() || proportion > P::one() {
            return Err(Error::OutOfRange(format!("`{:?}` is out of range 0..=1", proportion)));
        }
        Ok(Self { size, proportion })
    }
}

impl<N: Copy, P: Copy> Sample<N, P> for ProportionSample<N, P> {
    fn size(&self) -> N {
        self.size
    }

    fn p_hat(&self) -> P {
        self.proportion
    }
}

impl<N, P> From<NSuccessesSample<N>> for ProportionSample<N, P>
where
    N: Copy + Into<P>,
    P: Div<Output = P>,
{
    fn from(other: NSuccessesSample<N>) -> Self {
        Self {
            size: other.size(),
            proportion: other.p_hat(),
        }
    }
}
