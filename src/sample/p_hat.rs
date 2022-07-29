use std::fmt::Debug;

use num_traits::{One, Zero};

use crate::{Error, Result, Sample};

/// Represents a sample with its size and proportion.
pub struct PHatSample<N, P> {
    size: N,
    proportion: P,
}

impl<N, P> PHatSample<N, P>
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

impl<N: Copy, P: Copy> Sample<N, P> for PHatSample<N, P> {
    fn size(&self) -> N {
        self.size
    }

    fn p_hat(&self) -> P {
        self.proportion
    }
}
